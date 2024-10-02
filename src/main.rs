use std::process::exit;

use application::Application;
use state::State;
use storage::Storage;
use tracing::error;

mod application;
mod challenge;
mod config;
mod error_response;
mod middleware;
mod routes;
mod site;
mod state;
mod storage;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let application = match Application::new().await {
        Ok(app) => app,
        Err(e) => {
            error!("App couldnt start: {:?}", e);
            exit(1);
        }
    };

    match application.run().await {
        Ok(_) => (),
        Err(e) => {
            error!("App crashed: {:?}", e);
            exit(1);
        }
    }

    //let storage = storage::StorageProvider::new(&config);
}
