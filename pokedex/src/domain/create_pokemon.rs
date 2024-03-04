use super::entities::{PokemonName, PokemonNumber, PokemonTypes};

pub struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn execute(req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(number.into()),
        _ => Response::BadRequest,
    }
}

pub enum Response {
    Ok(u16),
    BadRequest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 42;
        let req = Request {
            number,
            name: "Pikachu".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(req);

        match res {
            Response::Ok(res_number) => assert_eq!(res_number, number),
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let req = Request {
            number: 42,
            name: "".to_string(),
            types: vec!["Electric".to_string()],
        };

        let res = execute(req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }
}
