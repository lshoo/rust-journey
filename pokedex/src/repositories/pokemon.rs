use std::sync::Mutex;

use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError>;
}

pub enum InsertError {
    Conflict,
    Unknown,
}

#[derive(Default)]
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
    ) -> Result<Pokemon, InsertError> {
        if self.error {
            return Err(InsertError::Unknown);
        }

        let mut pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(InsertError::Unknown),
        };

        if pokemons.iter().any(|p| p.number == number) {
            return Err(InsertError::Conflict);
        }

        let pokemon = Pokemon::new(number, name, types);
        pokemons.push(pokemon.clone());

        Ok(pokemon)
    }
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}
