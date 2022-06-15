use lazy_static::lazy_static;
use lazycell::AtomicLazyCell;
use serde::{Serialize, Deserialize};

lazy_static! {
    pub static ref CONFIG: AtomicLazyCell<AppConfig> = AtomicLazyCell::new();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub db_dir: String,
    pub confirmations: u32,
    pub lwd_url: String,
    pub poll_interval: u32,
    pub notify_host: String,
}

