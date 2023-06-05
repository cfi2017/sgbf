use std::sync::{Arc, RwLock};
use axum::extract::FromRef;
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
    pub(crate) config: Config,
}

impl AppState {
    pub fn new( config: Config) -> Self {
        Self {
            config,
        }
    }
}

impl FromRef<SharedState> for AppState {
    fn from_ref(input: &SharedState) -> Self {
        input.inner.read().unwrap().clone()
    }
}

