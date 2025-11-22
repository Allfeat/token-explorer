use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::response::{
    Sse,
    sse::{Event, KeepAlive},
};
use futures::{Stream, StreamExt};
use leptos::logging::log;
use subxt::{
    OnlineClient, SubstrateConfig,
    backend::chain_head::{ChainHeadBackend, ChainHeadBackendBuilder},
    lightclient::{ChainConfig, LightClient},
};

use tokio::sync::OnceCell;

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(
    runtime_metadata_path = "./artifacts/allfeat_metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]
pub mod allfeat {}

const ALLFEAT_SPEC: &str = include_str!("../allfeat.json");

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
    let (_, chain_rpc) = LightClient::relay_chain(chain_config)?;
    let backend: ChainHeadBackend<SubstrateConfig> =
        ChainHeadBackendBuilder::default().build_with_background_driver(chain_rpc.clone());
    let api = OnlineClient::<SubstrateConfig>::from_backend(Arc::new(backend)).await?;

    log!("Allfeat lightclient initialized.");

    Ok(api)
}

#[allow(unused)]
pub async fn sse_block_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let chain_api = chain_api().await;

    let blocks_sub = chain_api
        .blocks()
        .subscribe_finalized()
        .await
        .expect("Failed to subscribe");

    let stream = blocks_sub.map(|block_result| match block_result {
        Ok(block) => {
            let num = block.number();
            Ok(Event::default().data(num.to_string()))
        }
        Err(err) => {
            eprintln!("Erreur RPC: {}", err);
            Ok(Event::default().comment(format!("error: {}", err)))
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(10)))
}
