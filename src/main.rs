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
    
    //TODO: REWRITE TO HANDLE DIFFRENT STORAGE SERVERS
    let sites = match config.get_storage() {
        config::StorageTypeConfig::Memory(in_memory_config) => in_memory_config.get_sites(),
    };

    let storage = storage::MemoryStorage::new(sites);




    println!("{:?}", storage);
}
