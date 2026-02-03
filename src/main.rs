#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use token_app::app::{App, shell};
    use token_app::state::AppState;
    use tracing::error;
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    dotenvy::dotenv().ok();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    let client = match rpc::connect().await {
        Ok(c) => c,
        Err(e) => {
            error!("CRITICAL: Failed to connect to RPC endpoint: {e}");
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
        allocations_cache: std::sync::Arc::new(tokio::sync::RwLock::new(None)),
    };

    let app = Router::new()
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
mod rpc {
    use std::time::Duration;
    use subxt::OnlineClient;
    use subxt::backend::rpc::reconnecting_rpc_client::{ExponentialBackoff, RpcClient};
    use token_app::substrate::AllfeatClient;

    pub async fn connect() -> Result<AllfeatClient, Box<dyn std::error::Error + Send + Sync>> {
        let rpc_url = std::env::var("RPC_URL")
            .unwrap_or_else(|_| "wss://mainnet.rpc.allfeat.org".to_string());

        tracing::info!(target: "allfeat", "Connecting to RPC: {}", rpc_url);

        let rpc = RpcClient::builder()
            .retry_policy(
                ExponentialBackoff::from_millis(100)
                    .max_delay(Duration::from_secs(10))
                    .take(5),
            )
            .build(rpc_url)
            .await?;

        let api = OnlineClient::from_rpc_client(rpc).await?;

        tracing::info!(target: "allfeat", "Connected to Allfeat network");
        Ok(api)
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
