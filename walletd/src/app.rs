use std::path::Path;
use lazy_static::lazy_static;
use zcash_primitives::sapling::keys::FullViewingKey;
use lazycell::AtomicLazyCell;
use zcash_client_backend::encoding::decode_extended_full_viewing_key;
use zcash_primitives::consensus::Parameters;
use zcash_primitives::zip32::ExtendedFullViewingKey;
use crate::config::AppConfig;
use crate::db::Db;
use crate::NETWORK;

lazy_static! {
    pub static ref APP: AtomicLazyCell<App> = AtomicLazyCell::new();
}

pub struct App {
    pub store: Db,
    pub fvk: ExtendedFullViewingKey,
    pub config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let db_path = Path::new(&config.db_dir).join("wallet.db");
        let db = Db::open(db_path.as_path()).unwrap();
        let fvk = decode_extended_full_viewing_key(NETWORK.hrp_sapling_extended_full_viewing_key(), &config.fvk).unwrap().unwrap();
        App {
            store: db,
            fvk,
            config
        }
    }
}