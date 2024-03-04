use std::sync::Mutex;

use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub trait Repository: Send + Sync {
    fn insert(&self, number: PokemonNumber, name: PokemonName, types: PokemonTypes)
        -> InsertResult;
}

pub enum InsertResult {
    Ok(PokemonNumber),
    Conflict,
    Error,
}

pub struct InMemoryRepository {
    error: bool,
    pokemons: Mutex<Vec<Pokemon>>,
}

impl Repository for InMemoryRepository {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> InsertResult {
        if self.error {
            return InsertResult::Error;
        }

        let mut pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            Err(_) => return InsertResult::Error,
        };

        if pokemons.iter().any(|p| p.number == number) {
            return InsertResult::Conflict;
        }

        let number_clone = number.clone();
        let pokemon = Pokemon::new(number_clone, name, types);
        pokemons.push(pokemon);

        InsertResult::Ok(number)
    }
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            pokemons: Mutex::new(vec![]),
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}
