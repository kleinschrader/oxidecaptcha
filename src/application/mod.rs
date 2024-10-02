use crate::{
    config::Config,
    routes::{delete_challange, get_challange},
    state::State,
    storage::StorageProvider,
};
use anyhow::{Context, Result};
use axum::{
    routing::{delete, get},
    Router,
};
use tokio::net::TcpListener;

pub struct Application {
    listener: TcpListener,
    state: State,
}

impl Application {
    pub async fn new() -> Result<Self> {
        let config = Self::parse_config()?;
        let storage = StorageProvider::new(&config);

        let listener = Self::create_listener(&config).await?;

        let state = State::new(config, storage);

        Ok(Self { listener, state })
    }

    pub async fn run(self) -> Result<()> {
        let get_site_middleware = axum::middleware::from_fn_with_state(
            self.state.clone(),
            crate::middleware::get_site_middleware,
        );

        let get_challenge_middleware = axum::middleware::from_fn_with_state(
            self.state.clone(),
            crate::middleware::get_challenge_middleware,
        );

        let timeout_middleware = axum::middleware::from_fn(crate::middleware::timeout_middleware);

        let logging_middleware = axum::middleware::from_fn(crate::middleware::logging_middleware);

        let site_router = axum::Router::new()
            .route("/site/:siteId/challenge", get(get_challange))
            .route_layer(get_site_middleware.clone())
            .with_state(self.state.clone());

        let site_challenge_router = axum::Router::new()
            .route(
                "/site/:siteId/challenge/:challengeId",
                delete(delete_challange),
            )
            .route_layer(get_challenge_middleware)
            .with_state(self.state);

        let combined_router = Router::new()
            .merge(site_router)
            .merge(site_challenge_router)
            .layer(timeout_middleware)
            .layer(logging_middleware);

        axum::serve(self.listener, combined_router).await?;

        Ok(())
    }

    fn parse_config() -> Result<Config> {
        let config =
            std::fs::read_to_string("config.json").context("Unable to open config.json")?;
        let config: Config =
            serde_json::from_str(&config).context("Unable to parse config.json")?;

        Ok(config)
    }

    async fn create_listener(config: &Config) -> Result<TcpListener> {
        let listener = config.get_listen_socket();

        TcpListener::bind(listener)
            .await
            .context("Unable to bind listener")
    }
}
