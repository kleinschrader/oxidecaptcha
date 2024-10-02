use std::sync::Arc;

use crate::{config::Config, storage::StorageProvider};

#[derive(Debug, Clone)]
pub struct State(Arc<InnerState>);

#[derive(Debug)]
struct InnerState {
    config: Config,
    storage: StorageProvider,
}

impl State {
    pub fn new(config: Config, storage: StorageProvider) -> State {
        let inner = InnerState { config, storage };

        let inner = Arc::new(inner);

        Self(inner)
    }

    pub fn get_config(&self) -> &Config {
        &self.0.config
    }

    pub async fn get_storage(&self) -> &StorageProvider {
        &self.0.storage
    }
}
