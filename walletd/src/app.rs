use std::path::Path;
use std::sync::{Arc};
use lazy_static::lazy_static;
use lazycell::AtomicLazyCell;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
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
    pub store: Arc<std::sync::Mutex<Db>>,
    pub lwd_client: Arc<tokio::sync::Mutex<CompactTxStreamerClient<Channel>>>,
    pub fvk: ExtendedFullViewingKey,
    pub config: AppConfig,
}

impl App {
    pub async fn new(config: AppConfig) -> Self {
        let db_path = Path::new(&config.db_dir).join("wallet.db");
        let db = Db::open(db_path.as_path()).unwrap();
        let fvk = decode_extended_full_viewing_key(NETWORK.hrp_sapling_extended_full_viewing_key(), &config.fvk).unwrap().unwrap();
        let is_new = db.is_new().unwrap();
        let url = config.lwd_url.clone();
        let mut channel = tonic::transport::Channel::from_shared(url.clone()).unwrap();
        if url.starts_with("https") {
            let pem = include_bytes!("ca.pem");
            let ca = Certificate::from_pem(pem);
            let tls = ClientTlsConfig::new().ca_certificate(ca);
            channel = channel.tls_config(tls).unwrap();
        }
        let lwd_client = CompactTxStreamerClient::connect(channel).await.unwrap();
        let app = App {
            store: Arc::new(std::sync::Mutex::new(db)),
            lwd_client: Arc::new(tokio::sync::Mutex::new(lwd_client)),
            fvk,
            config
        };
        if is_new {
            Db::create_account(app.store.clone(), None, &app.fvk).await.unwrap();
        }
        app
    }
}

pub fn get_appstore() -> &'static App {
    APPSTORE.borrow().unwrap()
}
