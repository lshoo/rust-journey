use std::sync::Arc;

use dialoguer::{Input, MultiSelect};

use crate::{domain::create_pokemon, repositories::pokemon::Repository};

pub fn run(repo: Arc<dyn Repository>) {
    let number = prompt_number();
    let name = prompt_name();
    let types = prompt_types();

    let req = match (number, name, types) {
        (Ok(number), Ok(name), Ok(types)) => create_pokemon::Request {
            number,
            name,
            types,
        },
        _ => {
            println!("An error occurred during the prompt!");
            return;
        }
    };

    match create_pokemon::execute(repo, req) {
        Ok(res) => println!(
            "{:#?}",
            Response {
                number: res.number,
                name: res.name,
                types: res.types,
            }
        ),
        Err(create_pokemon::Error::BadRequest) => println!("The request is invalid"),
        Err(create_pokemon::Error::Conflict) => println!("The Pokemon already exists!"),
        Err(create_pokemon::Error::Unknown) => println!("An unknown error occurred"),
    }
}

#[derive(Debug)]
pub struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn prompt_number() -> Result<u16, ()> {
    match Input::new().with_prompt("Pokemon number").interact_text() {
        Ok(number) => Ok(number),
        _ => Err(()),
    }
}

pub fn prompt_name() -> Result<String, ()> {
    match Input::new().with_prompt("Pokemon name").interact_text() {
        Ok(name) => Ok(name),
        _ => Err(()),
    }
}

pub fn prompt_types() -> Result<Vec<String>, ()> {
    let types = vec!["Electric", "Fire"];

    match MultiSelect::new()
        .with_prompt("Pokemon types")
        .items(&types)
        .interact()
    {
        Ok(indexes) => Ok(indexes
            .into_iter()
            .map(|idx| String::from(types[idx]))
            .collect()),
        _ => Err(()),
    }
}
