use std::sync::Arc;

use serde::Deserialize;

use crate::{domain::create_pokemon, repositories::pokemon::Repository};

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
        create_pokemon::Response::Ok(number) => rouille::Response::json(&number),
        create_pokemon::Response::Conflict => rouille::Response::from(Status::Conflict),
        create_pokemon::Response::Error => rouille::Response::from(Status::InternalServerError),
        create_pokemon::Response::BadRequest => rouille::Response::from(Status::BadRequest),
    }
}

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}
