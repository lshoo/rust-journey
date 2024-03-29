use std::sync::Arc;

use serde::Serialize;

use crate::{domain::fetch_all_pokemons, repositories::pokemon::Repository};

use super::Status;

pub fn serve(repo: Arc<dyn Repository>) -> rouille::Response {
    match fetch_all_pokemons::execute(repo) {
        Ok(res) => rouille::Response::json(
            &res.into_iter()
                .map(|p| Response {
                    number: p.number,
                    name: p.name,
                    types: p.types,
                })
                .collect::<Vec<Response>>(),
        ),
        Err(fetch_all_pokemons::Error::Unknown) => {
            rouille::Response::from(Status::InternalServerError)
        }
    }
}

#[derive(Serialize)]
pub struct Response {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}
