use std::sync::Arc;

use serde::Serialize;

use crate::{domain::fetch_pokemon, repositories::pokemon::Repository};

use super::Status;

pub fn serve(repo: Arc<dyn Repository>, number: u16) -> rouille::Response {
    let req = fetch_pokemon::Request { number };

    match fetch_pokemon::execute(repo, req) {
        Ok(fetch_pokemon::Response {
            number,
            name,
            types,
        }) => rouille::Response::json(&Response {
            number,
            name,
            types,
        }),
        Err(fetch_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
        Err(fetch_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
        Err(fetch_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
    }
}

#[derive(Serialize)]
pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}
