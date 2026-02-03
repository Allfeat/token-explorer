use leptos::server_fn::codec::StreamingText;
use leptos::server_fn::codec::TextStream;
#[cfg(feature = "ssr")]
use ssr::*;

use leptos::prelude::*;
use leptos::server;

use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "ssr")]
pub mod state;
#[cfg(feature = "ssr")]
pub mod substrate;

pub mod app;
pub mod components;
mod pages;
pub mod utils;

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

#[server(output = StreamingText)]
pub async fn get_block_number_stream() -> Result<TextStream, ServerFnError> {
    use futures::StreamExt;
    use tracing::error;

    let chain_api = get_chain_api().await?;

    let blocks_sub = chain_api
        .blocks()
        .subscribe_finalized()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to subscribe to blocks: {e}")))?;

    let stream = blocks_sub.map(|block_result| match block_result {
        Ok(block) => {
            let num = block.header().number;
            Ok(num.to_string())
        }
        Err(err) => {
            error!("RPC Error: {err}");
            Ok("Error".to_string())
        }
    });

    Ok(TextStream::new(stream))
}

#[server]
pub async fn get_allocations() -> Result<Vec<EnvelopeAllocation>, ServerFnError> {
    use futures::future::try_join_all;
    use std::time::Duration;

    const CACHE_TTL: Duration = Duration::from_secs(300); // 5 minutes

    let state = expect_context::<AppState>();

    // Check cache first
    {
        let cache = state.allocations_cache.read().await;
        if let Some(cached) = &*cache {
            if cached.cached_at.elapsed() < CACHE_TTL {
                return Ok(cached.data.clone());
            }
        }
    }

    // Cache miss or expired - fetch fresh data
    let chain_api = get_chain_api().await?;

    // Get block reference once and reuse for all queries (batching optimization)
    let block = chain_api.blocks().at_latest().await?;
    let block_ref = block.reference();

    // Parallelize all envelope queries using centralized ENVELOPES config
    let futures = ENVELOPES
        .iter()
        .map(|(id, name)| get_alloc_config_of(&chain_api, &block_ref, id, name));

    let results = try_join_all(futures).await?;

    // Update cache
    {
        let mut cache = state.allocations_cache.write().await;
        *cache = Some(CachedData {
            data: results.clone(),
            cached_at: std::time::Instant::now(),
        });
    }

    Ok(results)
}

#[server]
pub async fn get_epoch_duration() -> Result<u32, ServerFnError> {
    let chain_api = get_chain_api().await?;

    let query = substrate::allfeat::constants()
        .token_allocation()
        .epoch_duration();

    Ok(chain_api.constants().at(&query)?)
}

#[server]
pub async fn get_allocations_of(id: String) -> Result<Vec<Allocation>, ServerFnError> {
    let chain_api = get_chain_api().await?;

    let query = substrate::allfeat::storage()
        .token_allocation()
        .allocations_iter();

    // Get block reference to ensure consistency across queries
    let block = chain_api.blocks().at_latest().await?;
    let block_ref = block.reference();
    let storage = chain_api.storage().at(block_ref.clone());
    let mut allocs_iter = storage.iter(query).await?;

    let mut allocs: Vec<Allocation> = vec![];

    while let Some(Ok(kv)) = allocs_iter.next().await {
        if format_ss58(&kv.value.beneficiary) == id {
            allocs.push(Allocation {
                envelope: get_alloc_config_of(
                    &chain_api,
                    &block_ref,
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
    let chain_api = get_chain_api().await?;

    let query = substrate::allfeat::storage().balances().total_issuance();

    let total = chain_api
        .storage()
        .at_latest()
        .await?
        .fetch(&query)
        .await?
        .ok_or_else(|| ServerFnError::new("Total issuance not found on chain"))?;

    Ok(total)
}

#[server]
pub async fn get_circulating_supply() -> Result<u128, ServerFnError> {
    let chain_api = get_chain_api().await?;

    let distributed_query = substrate::allfeat::storage()
        .token_allocation()
        .envelope_distributed_iter();
    let allocations_query = substrate::allfeat::storage()
        .token_allocation()
        .allocations_iter();

    // Use a single block reference for both iterators (consistency + fewer RPC calls)
    let storage = chain_api.storage().at_latest().await?;

    let mut distributed_iter = storage.iter(distributed_query).await?;

    let mut total_distributed: u128 = 0;
    while let Some(Ok(kv)) = distributed_iter.next().await {
        total_distributed += kv.value
    }

    let mut allocations_iter = storage.iter(allocations_query).await?;

    let mut total_in_vesting: u128 = 0;
    while let Some(Ok(kv)) = allocations_iter.next().await {
        total_in_vesting += kv.value.vested_total.saturating_sub(kv.value.released)
    }

    let distributed_less_in_vesting = total_distributed.saturating_sub(total_in_vesting);

    Ok(distributed_less_in_vesting)
}

#[server]
pub async fn get_balance_of(id: String) -> Result<Balances, ServerFnError> {
    let chain_api = get_chain_api().await?;

    let account_id =
        AccountId32::from_str(&id).map_err(|_| ServerFnError::new("Invalid address format"))?;

    let query = substrate::allfeat::storage().system().account(account_id);

    let account_info = chain_api.storage().at_latest().await?.fetch(&query).await?;

    // Return zero balances if account doesn't exist on chain
    match account_info {
        Some(info) => Ok(Balances {
            free: info.data.free,
            reserved: info.data.reserved,
            frozen: info.data.frozen,
        }),
        None => Ok(Balances {
            free: 0,
            reserved: 0,
            frozen: 0,
        }),
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    pub use super::state::AppState;
    pub use super::state::CachedData;
    pub use super::substrate::AllfeatClient;
    pub use super::substrate::allfeat::runtime_types::pallet_token_allocation::EnvelopeId;
    use super::*;
    pub use std::str::FromStr;
    pub use subxt::OnlineClient;
    pub use subxt::SubstrateConfig;
    pub use subxt::utils::AccountId32;

    /// Centralized envelope configuration - single source of truth
    pub const ENVELOPES: &[(EnvelopeId, &str)] = &[
        (EnvelopeId::Airdrop, "Airdrop"),
        (EnvelopeId::CommunityRewards, "Community Rewards"),
        (EnvelopeId::Private1, "Private Funding #1"),
        (EnvelopeId::Private2, "Private Funding #2"),
        (EnvelopeId::Public2, "Public Funding #2"),
        (EnvelopeId::Public4, "Public Funding #4"),
        (EnvelopeId::Public1, "Public Funding #1"),
        (EnvelopeId::Public3, "Public Funding #3"),
        (EnvelopeId::Teams, "Teams"),
        (EnvelopeId::Reserve, "Reserve"),
        (EnvelopeId::Listing, "Listing (CEX/DEX)"),
        (EnvelopeId::ResearchDevelopment, "Research & Development"),
        (EnvelopeId::KoL, "KoL Funding"),
    ];

    /// Encode an AccountId32 to SS58 format with the Allfeat prefix (440)
    pub fn format_ss58(account: &AccountId32) -> String {
        use crate::utils::SS58_PREFIX;
        use blake2::{Blake2b512, Digest};

        const SS58_PREFIX_BYTES: &[u8] = b"SS58PRE";

        let public_key: &[u8; 32] = account.as_ref();

        // For prefix > 63, use two-byte encoding
        let prefix_bytes = [
            ((SS58_PREFIX & 0x00FC) as u8 >> 2) | 0x40,
            ((SS58_PREFIX >> 8) as u8) | ((SS58_PREFIX & 0x0003) as u8) << 6,
        ];

        // Compute checksum: blake2b-512 of (SS58PRE || prefix || pubkey)
        let mut hasher = Blake2b512::new();
        hasher.update(SS58_PREFIX_BYTES);
        hasher.update(&prefix_bytes);
        hasher.update(public_key);
        let hash = hasher.finalize();

        // Build the full address: prefix (2 bytes) + pubkey (32 bytes) + checksum (2 bytes)
        let mut address = Vec::with_capacity(36);
        address.extend_from_slice(&prefix_bytes);
        address.extend_from_slice(public_key);
        address.extend_from_slice(&hash[0..2]);

        bs58::encode(address).into_string()
    }

    pub async fn get_chain_api() -> Result<AllfeatClient, ServerFnError> {
        use axum::extract::State;
        use leptos_axum::extract_with_state;

        let state = expect_context::<AppState>();
        let State(client): State<AppState> = extract_with_state(&state).await?;

        Ok(client.client)
    }

    pub fn envelope_to_str(envelope: &EnvelopeId) -> &'static str {
        ENVELOPES
            .iter()
            .find(|(id, _)| id == envelope)
            .map(|(_, name)| *name)
            .unwrap_or("Unknown")
    }

    pub async fn get_alloc_config_of(
        chain_api: &OnlineClient<SubstrateConfig>,
        block_ref: &subxt::blocks::BlockRef<subxt::utils::H256>,
        envelope: &EnvelopeId,
        name: &str,
    ) -> Result<EnvelopeAllocation, ServerFnError> {
        let query = substrate::allfeat::storage()
            .token_allocation()
            .envelopes(envelope.clone());
        let query_distributed = substrate::allfeat::storage()
            .token_allocation()
            .envelope_distributed(envelope.clone());

        // Reuse the same block reference for both queries
        let storage = chain_api.storage().at(block_ref.clone());

        let (res, res_distributed) =
            tokio::try_join!(storage.fetch(&query), storage.fetch(&query_distributed))?;

        let res =
            res.ok_or_else(|| ServerFnError::new(format!("Envelope config not found for {name}")))?;
        let res_distributed = res_distributed.unwrap_or(0);

        Ok(EnvelopeAllocation {
            id: name.to_lowercase().to_string(),
            name: name.to_string(),
            total_cap: res.total_cap,
            unique_beneficiary: res.unique_beneficiary.map(|addr| format_ss58(&addr)),
            cliff: res.cliff,
            vesting_duration: res.vesting_duration,
            distributed: res_distributed,
            upfront_rate: res.upfront_rate.0,
        })
    }
}
