#[macro_use]
extern crate rocket;

use rocket::{Request, response, Response};
use rocket::http::Status;
use rocket::response::Responder;
use thiserror::Error;
use zcash_primitives::consensus::Network;
use zcash_primitives::consensus::Network::MainNetwork;

#[path = "generated/cash.z.wallet.sdk.rpc.rs"]
pub mod lw_rpc;

pub mod config;
pub mod rpc;
pub mod data;
mod app;
mod db;
mod chain;

pub const NETWORK: Network = MainNetwork;
pub use app::{App, APPSTORE, get_appstore};
pub use chain::scan_blocks;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Blockchain reorganization")]
    ReorgDetected,

    #[error(transparent)]
    Tonic(#[from] tonic::Status),

    #[error(transparent)]
    SQL(#[from] rusqlite::Error),

    #[error(transparent)]
    Rocket(#[from] rocket::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error)
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let error = self.to_string();
        Response::build_from(error.respond_to(req)?)
            .status(Status::InternalServerError)
            .ok()
    }
}
