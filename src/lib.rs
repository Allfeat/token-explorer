#[cfg(feature = "ssr")]
use std::str::FromStr;

use leptos::prelude::*;
use leptos::server;

use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "ssr")]
use subxt::OnlineClient;
#[cfg(feature = "ssr")]
use subxt::SubstrateConfig;

#[cfg(feature = "ssr")]
use crate::substrate::allfeat::runtime_types::pallet_token_allocation::EnvelopeId;
#[cfg(feature = "ssr")]
use subxt::utils::AccountId32;

#[cfg(feature = "ssr")]
mod substrate;

pub mod components;
pub mod utils;

pub mod app;
mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Balances {
    pub free: u128,
    pub reserved: u128,
    pub frozen: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeAllocation {
    pub id: String,
    pub name: String,
    pub total_cap: u128,
    pub upfront_rate: u8,
    pub cliff: u32,
    pub vesting_duration: u32,
    pub unique_beneficiary: Option<String>,
    pub distributed: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Allocation {
    pub envelope: EnvelopeAllocation,
    pub total: u128,
    pub upfront: u128,
    pub vested_total: u128,
    pub released: u128,
    pub start: u32,
}

#[server]
pub async fn get_allocations() -> Result<Vec<EnvelopeAllocation>, ServerFnError> {
    let chain_api = substrate::chain_api().await;

    // FIXME: https://github.com/paritytech/subxt/issues/1743 not using an iter on storage
    // cause of this
    let v = vec![
        get_alloc_config_of(chain_api, &EnvelopeId::Airdrop, "Airdrop").await?,
        get_alloc_config_of(
            chain_api,
            &EnvelopeId::CommunityRewards,
            "Community Rewards",
        )
        .await?,
        get_alloc_config_of(chain_api, &EnvelopeId::Private1, "Private Funding #1").await?,
        get_alloc_config_of(chain_api, &EnvelopeId::Private2, "Private Funding #2").await?,
        get_alloc_config_of(chain_api, &EnvelopeId::Seed, "Seed Funding").await?,
        get_alloc_config_of(chain_api, &EnvelopeId::SerieA, "Serie A Funding").await?,
        get_alloc_config_of(chain_api, &EnvelopeId::ICO1, "ICO #1").await?,
        get_alloc_config_of(chain_api, &EnvelopeId::ICO2, "ICO #2").await?,
        get_alloc_config_of(chain_api, &EnvelopeId::Founders, "Founders").await?,
        get_alloc_config_of(chain_api, &EnvelopeId::Reserve, "Reserve").await?,
        get_alloc_config_of(chain_api, &EnvelopeId::Exchanges, "Exchanges (CEX/DEX)").await?,
        get_alloc_config_of(
            chain_api,
            &EnvelopeId::ResearchDevelopment,
            "Research & Development",
        )
        .await?,
        get_alloc_config_of(chain_api, &EnvelopeId::KoL, "KoL Funding").await?,
    ];

    Ok(v)
}

#[server]
pub async fn get_epoch_duration() -> Result<u32, ServerFnError> {
    let chain_api = substrate::chain_api().await;

    let query = substrate::allfeat::constants()
        .token_allocation()
        .epoch_duration();

    Ok(chain_api.constants().at(&query)?)
}

#[server]
pub async fn get_allocations_of(id: String) -> Result<Vec<Allocation>, ServerFnError> {
    let chain_api = substrate::chain_api().await;

    let query = substrate::allfeat::storage()
        .token_allocation()
        .allocations_iter();

    let mut allocs_iter = chain_api.storage().at_latest().await?.iter(query).await?;

    let mut allocs: Vec<Allocation> = vec![];

    while let Some(Ok(kv)) = allocs_iter.next().await {
        if kv.value.beneficiary.to_string() == id {
            allocs.push(Allocation {
                envelope: get_alloc_config_of(
                    chain_api,
                    &kv.value.envelope,
                    envelope_to_str(&kv.value.envelope),
                )
                .await?,
                total: kv.value.total,
                upfront: kv.value.upfront,
                released: kv.value.released,
                vested_total: kv.value.vested_total,
                start: kv.value.start,
            });
        }
    }

    Ok(allocs)
}

#[server]
pub async fn get_total_issuance() -> Result<u128, ServerFnError> {
    let chain_api = substrate::chain_api().await;
    let query = substrate::allfeat::storage().balances().total_issuance();

    Ok(chain_api
        .storage()
        .at_latest()
        .await?
        .fetch(&query)
        .await?
        .unwrap())
}

#[server]
pub async fn get_circulating_supply() -> Result<u128, ServerFnError> {
    let chain_api = substrate::chain_api().await;

    let distributed_query = substrate::allfeat::storage()
        .token_allocation()
        .envelope_distributed_iter();
    let allocations_query = substrate::allfeat::storage()
        .token_allocation()
        .allocations_iter();

    let mut distributed_iter = chain_api
        .storage()
        .at_latest()
        .await?
        .iter(distributed_query)
        .await?;

    let mut total_distributed: u128 = 0;
    while let Some(Ok(kv)) = distributed_iter.next().await {
        total_distributed += kv.value
    }

    let mut allocations_iter = chain_api
        .storage()
        .at_latest()
        .await?
        .iter(allocations_query)
        .await?;

    let mut total_in_vesting: u128 = 0;
    while let Some(Ok(kv)) = allocations_iter.next().await {
        total_in_vesting += kv.value.vested_total.saturating_sub(kv.value.released)
    }

    let distributed_less_in_vesting = total_distributed.saturating_sub(total_in_vesting);

    Ok(distributed_less_in_vesting)
}

#[server]
pub async fn get_balance_of(id: String) -> Result<Balances, ServerFnError> {
    let chain_api = substrate::chain_api().await;
    let query = substrate::allfeat::storage()
        .system()
        .account(AccountId32::from_str(&id).expect("Valid address"));

    let account_info = chain_api
        .storage()
        .at_latest()
        .await?
        .fetch(&query)
        .await?
        .unwrap();

    Ok(Balances {
        free: account_info.data.free,
        reserved: account_info.data.reserved,
        frozen: account_info.data.frozen,
    })
}

#[cfg(feature = "ssr")]
pub fn envelope_to_str(envelope: &EnvelopeId) -> &str {
    match envelope {
        EnvelopeId::Airdrop => "Airdrop",
        EnvelopeId::CommunityRewards => "Community Rewards",
        EnvelopeId::Private1 => "Private Funding #1",
        EnvelopeId::Private2 => "Private Funding #2",
        EnvelopeId::Seed => "Seed Funding",
        EnvelopeId::SerieA => "Serie A Funding",
        EnvelopeId::ICO1 => "ICO #1",
        EnvelopeId::ICO2 => "ICO #2",
        EnvelopeId::Founders => "Founders",
        EnvelopeId::Reserve => "Reserve",
        EnvelopeId::Exchanges => "Exchanges (CEX/DEX)",
        EnvelopeId::ResearchDevelopment => "Research & Development",
        EnvelopeId::KoL => "KoL Funding",
    }
}

#[cfg(feature = "ssr")]
pub async fn get_alloc_config_of(
    chain_api: &OnlineClient<SubstrateConfig>,
    envelope: &EnvelopeId,
    name: &str,
) -> Result<EnvelopeAllocation, ServerFnError> {
    let query = substrate::allfeat::storage()
        .token_allocation()
        .envelopes(envelope.clone());
    let query_distributed = substrate::allfeat::storage()
        .token_allocation()
        .envelope_distributed(envelope.clone());

    let res = chain_api
        .storage()
        .at_latest()
        .await?
        .fetch(&query)
        .await?
        .unwrap();
    let res_distributed = chain_api
        .storage()
        .at_latest()
        .await?
        .fetch(&query_distributed)
        .await?
        .unwrap();

    Ok(EnvelopeAllocation {
        id: name.to_lowercase().to_string(),
        name: name.to_string(),
        total_cap: res.total_cap,
        unique_beneficiary: res.unique_beneficiary.map(|addr| addr.to_string()),
        cliff: res.cliff,
        vesting_duration: res.vesting_duration,
        distributed: res_distributed,
        upfront_rate: res.upfront_rate.0,
    })
}
