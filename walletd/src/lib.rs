#[macro_use]
extern crate rocket;

use rocket::{Request, response, Response};
use rocket::http::Status;
use rocket::response::Responder;
use thiserror::Error;

pub mod config;
pub mod rpc;
pub mod data;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
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
