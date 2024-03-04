pub mod create_pokemon;
pub mod health;

use std::sync::Arc;

use rouille::router;
use serde::Serialize;

use crate::repositories::pokemon::Repository;

pub fn serve(url: &str, repo: Arc<dyn Repository>) {
    rouille::start_server(url, move |req| {
        router!(req,
            (GET) (/health) => {
                health::serve()
            },
            (POST) (/) => {
                create_pokemon::serve(req, repo.clone())
            },
            _ => {
                rouille::Response::from(Status::NotFound)
            }
        )
    })
}

enum Status {
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
}

impl From<Status> for rouille::Response {
    fn from(status: Status) -> Self {
        let status_code = match status {
            Status::BadRequest => 400,
            Status::NotFound => 404,
            Status::Conflict => 409,
            Status::InternalServerError => 500,
        };

        Self {
            status_code,
            headers: vec![],
            data: rouille::ResponseBody::empty(),
            upgrade: None,
        }
    }
}

#[derive(Serialize)]
struct Response {
    message: String,
}