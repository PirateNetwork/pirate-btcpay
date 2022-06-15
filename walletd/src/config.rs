use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub db_dir: String,
    pub starting_height: u32,
    pub confirmations: u32,
    pub lwd_url: String,
    pub poll_interval: u32,
    pub notify_host: String,
    pub fvk: String,
}

