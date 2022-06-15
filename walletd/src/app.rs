use std::path::Path;
use std::sync::Arc;
use lazy_static::lazy_static;
use zcash_primitives::sapling::keys::FullViewingKey;
use lazycell::AtomicLazyCell;
use tokio::sync::Mutex;
use tonic::transport::Channel;
use zcash_client_backend::encoding::decode_extended_full_viewing_key;
use zcash_primitives::consensus::Parameters;
use zcash_primitives::zip32::ExtendedFullViewingKey;
use crate::config::AppConfig;
use crate::db::Db;
use crate::lw_rpc::compact_tx_streamer_client::CompactTxStreamerClient;
use crate::NETWORK;

lazy_static! {
    pub static ref APPSTORE: AtomicLazyCell<App> = AtomicLazyCell::new();
}

pub struct App {
    pub store: Arc<Mutex<Db>>,
    pub lwd_client: Arc<Mutex<CompactTxStreamerClient<Channel>>>,
    pub fvk: ExtendedFullViewingKey,
    pub config: AppConfig,
}

impl App {
    pub async fn new(config: AppConfig) -> Self {
        let db_path = Path::new(&config.db_dir).join("wallet.db");
        let db = Db::open(db_path.as_path()).unwrap();
        let fvk = decode_extended_full_viewing_key(NETWORK.hrp_sapling_extended_full_viewing_key(), &config.fvk).unwrap().unwrap();
        let lwd_client = CompactTxStreamerClient::connect(config.lwd_url.clone()).await.unwrap();
        App {
            store: Arc::new(Mutex::new(db)),
            lwd_client: Arc::new(Mutex::new(lwd_client)),
            fvk,
            config
        }
    }
}

pub fn get_appstore() -> &'static App {
    APPSTORE.borrow().unwrap()
}
