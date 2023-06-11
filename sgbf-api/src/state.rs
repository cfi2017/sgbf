use std::sync::{Arc, RwLock};
use axum::extract::FromRef;
use sgbf_client::client::axum::{AuthCache};
use crate::cache::{Cache, CacheRef};
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct SharedState {
    pub(crate) inner: Arc<RwLock<AppState>>,
}

impl SharedState {
    pub fn build(state: AppState) -> Self {
        Self { inner: Arc::new(RwLock::new(state)) }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) auth_cache: AuthCache,
    pub(crate) config: Config,
    pub(crate) cache: CacheRef
}

impl AppState {
    pub fn new(auth_cache: AuthCache, cache: CacheRef, config: Config) -> Self {
        Self {
            auth_cache,
            cache,
            config,
        }
    }
}

impl FromRef<SharedState> for AppState {
    fn from_ref(input: &SharedState) -> Self {
        input.inner.read().unwrap().clone()
    }
}

impl FromRef<SharedState> for CacheRef {
    fn from_ref(input: &SharedState) -> Self {
        input.inner.read().unwrap().clone().cache
    }
}

impl FromRef<SharedState> for AuthCache {
    fn from_ref(input: &SharedState) -> Self {
        input.inner.read().unwrap().clone().auth_cache
    }
}

