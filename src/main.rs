use std::process::exit;

use application::Application;
use storage::Storage;
use tokio::net::TcpListener;
use tracing::{error, info};
use uuid::uuid;
use state::State;

mod challenge;
mod config;
mod site;
mod storage;
mod routes;
mod application;
mod state;
mod errorResponse;
mod middleware;

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt().init();
    
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
