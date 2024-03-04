use crate::repositories::pokemon::{InsertResult, Repository};

use super::entities::{PokemonName, PokemonNumber, PokemonTypes};

use std::{convert::TryFrom, sync::Arc};

pub struct Request {
    pub number: u16,
    pub name: String,
    pub types: Vec<String>,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
            InsertResult::Ok(number) => Response::Ok(number.into()),
            InsertResult::Conflict => Response::Conflict,
            InsertResult::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}

pub enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

#[cfg(test)]
mod tests {
    use crate::repositories::pokemon::InMemoryRepository;

    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let repo = Arc::new(InMemoryRepository::new());

        let number = 42;
        let req = Request {
            number,
            name: "Pikachu".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(repo, req);

        match res {
            Response::Ok(res_number) => assert_eq!(res_number, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let repo = Arc::new(InMemoryRepository::new());
        let req = Request {
            number: 42,
            name: "".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(repo, req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let number = 25.try_into().unwrap();
        let name = "pikachu".to_string().try_into().unwrap();
        let types = vec!["Electric".to_string()].try_into().unwrap();

        let repo = Arc::new(InMemoryRepository::new());
        repo.insert(number, name, types);

        let req = Request {
            number: 25,
            name: "Charmander".to_string(),
            types: vec!["Fire".to_string()],
        };

        let res = execute(repo, req);

        match res {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_error_when_an_unexpected_error_happens() {
        let number = 25.try_into().unwrap();
        let name = "pikachu".to_string().try_into().unwrap();
        let types = vec!["Electric".to_string()].try_into().unwrap();

        let repo = Arc::new(InMemoryRepository::new().with_error());
        repo.insert(number, name, types);

        let req = Request {
            number: 25,
            name: "Pikachu".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(repo, req);

        match res {
            Response::Error => {}
            _ => unreachable!(),
        }
    }
}
