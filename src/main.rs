use storage::Storage;
use uuid::uuid;

mod challenge;
mod config;
mod site;
mod storage;

#[tokio::main]
async fn main() {
    let config = std::fs::read_to_string("config.json")
        .expect("Unable to load config.json");
    let config: config::Config = serde_json::from_str(&config)
        .expect("Unable to parse config");

    let storage = storage::StorageProvider::new(&config);

    let site = storage.get_site(&uuid!("e169138a-44bc-4685-89ac-827f62e6d070")).await.unwrap();

    let challenge = site.generate_challenge();
    let challenge = challenge.pluck();
    let challenge = challenge.unpluck(&site);


    println!("{}", serde_json::to_string(&challenge).unwrap());
}
