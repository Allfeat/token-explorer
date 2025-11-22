use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use super::substrate::AllfeatClient;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub client: AllfeatClient,
}
