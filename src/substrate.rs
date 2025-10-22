use leptos::{logging::log, prelude::ServerFnError};
use subxt::{
    lightclient::{ChainConfig, LightClient},
    OnlineClient, SubstrateConfig,
};

use tokio::sync::OnceCell;

use crate::{
    substrate::allfeat::runtime_types::pallet_token_allocation::EnvelopeId, EnvelopeAllocation,
};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(
    runtime_metadata_path = "./artifacts/allfeat_metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod allfeat {}

const ALLFEAT_SPEC: &str = include_str!("../allfeat_dev.json");

static CHAIN_API: OnceCell<OnlineClient<SubstrateConfig>> = OnceCell::const_new();

pub async fn chain_api() -> &'static OnlineClient<SubstrateConfig> {
    CHAIN_API
        .get_or_init(async || start_lightclient().await.unwrap())
        .await
}

pub async fn start_lightclient() -> Result<OnlineClient<SubstrateConfig>, Box<dyn std::error::Error>>
{
    log!("Starting Allfeat lightclient...");

    let chain_config = ChainConfig::chain_spec(ALLFEAT_SPEC);

    // Start the light client up, establishing a connection to the local node.
    let (_light_client, chain_rpc) = LightClient::relay_chain(chain_config)?;
    let api = OnlineClient::<SubstrateConfig>::from_rpc_client(chain_rpc).await?;

    log!("Allfeat lightclient initialized.");

    Ok(api)
}

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

pub async fn get_alloc_config_of(
    chain_api: &OnlineClient<SubstrateConfig>,
    envelope: &EnvelopeId,
    name: &str,
) -> Result<EnvelopeAllocation, ServerFnError> {
    let query = allfeat::storage()
        .token_allocation()
        .envelopes(envelope.clone());
    let query_distributed = allfeat::storage()
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
