use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    domain::create_pokemon::{self, Error},
    repositories::pokemon::Repository,
};

use super::Status;

pub fn serve(req: &rouille::Request, repo: Arc<dyn Repository>) -> rouille::Response {
    let req = match rouille::input::json_input::<Request>(req) {
        Ok(req) => create_pokemon::Request {
            number: req.number,
            name: req.name,
            types: req.types,
        },
        _ => return rouille::Response::from(Status::BadRequest),
    };

    match create_pokemon::execute(repo, req) {
        Ok(create_pokemon::Response {
            number,
            name,
            types,
        }) => rouille::Response::json(&Response {
            number,
            name,
            types,
        }),
        Err(Error::Conflict) => rouille::Response::from(Status::Conflict),
        Err(Error::Unknown) => rouille::Response::from(Status::InternalServerError),
        Err(Error::BadRequest) => rouille::Response::from(Status::BadRequest),
    }
}

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

#[derive(Serialize)]
pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}
