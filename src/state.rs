use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use super::substrate::AllfeatClient;
use crate::EnvelopeAllocation;

/// Cached data with timestamp for TTL validation
#[derive(Clone, Debug)]
pub struct CachedData<T> {
    pub data: T,
    pub cached_at: Instant,
}

/// Type alias for the allocations cache
pub type AllocationsCache = Arc<RwLock<Option<CachedData<Vec<EnvelopeAllocation>>>>>;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub client: AllfeatClient,
    pub allocations_cache: AllocationsCache,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("leptos_options", &self.leptos_options)
            .field("client", &self.client)
            .field("allocations_cache", &"<RwLock>")
            .finish()
    }
}
