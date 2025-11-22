#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum::routing::get;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use token_app::app::{App, shell};
    use token_app::state::AppState;

    let client = match substrate::start_lightclient().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("CRITICAL: Failed to start Light Client: {}", e);
            std::process::exit(1);
        }
    };

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        client,
    };

    let app = Router::new()
        .route("/api/sse/blocks", get(substrate::sse_block_handler))
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
mod substrate {
    use std::{convert::Infallible, sync::Arc, time::Duration};

    use axum::{
        extract::State,
        response::{
            Sse,
            sse::{Event, KeepAlive},
        },
    };
    use futures::{Stream, StreamExt};
    use subxt::{
        OnlineClient, SubstrateConfig,
        backend::chain_head::ChainHeadBackendBuilder,
        lightclient::{ChainConfig, LightClient},
    };

    pub type AllfeatClient = OnlineClient<SubstrateConfig>;

    const ALLFEAT_SPEC: &str = include_str!("../allfeat.json");

    pub async fn start_lightclient() -> Result<AllfeatClient, Box<dyn std::error::Error>> {
        println!("🚀 Starting Allfeat lightclient...");
        let chain_config = ChainConfig::chain_spec(ALLFEAT_SPEC);

        let (_, chain_rpc) = LightClient::relay_chain(chain_config)?;

        let backend = ChainHeadBackendBuilder::default().build_with_background_driver(chain_rpc);

        let api = AllfeatClient::from_backend(Arc::new(backend)).await?;

        println!("✅ Allfeat lightclient synced and ready.");
        Ok(api)
    }

    pub async fn sse_block_handler(
        State(client): State<AllfeatClient>,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        // Plus besoin d'attendre l'initialisation, le client est déjà prêt !
        let blocks_sub = client
            .blocks()
            .subscribe_finalized()
            .await
            .expect("Failed to subscribe"); // Gestion d'erreur à améliorer en prod

        let stream = blocks_sub.map(|block_result| match block_result {
            Ok(block) => {
                let num = block.header().number;
                Ok(Event::default().data(num.to_string()))
            }
            Err(err) => {
                eprintln!("RPC Error: {}", err);
                Ok(Event::default().comment("error"))
            }
        });

        Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(10)))
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
