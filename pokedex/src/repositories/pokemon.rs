use std::sync::{Arc, Mutex};

use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub trait Repository: Send + Sync {
    fn insert(
        &self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> Result<Pokemon, InsertError>;

    fn fetch_all(&self) -> Result<Vec<Pokemon>, FetchAllError>;

    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, FetchOneError>;

    fn delete(&self, number: PokemonNumber) -> Result<(), DeleteError>;
}

pub enum InsertError {
    Conflict,
    Unknown,
}

pub enum FetchAllError {
    Unknown,
}

pub enum FetchOneError {
    NotFound,
    Unknown,
}

pub enum DeleteError {
    NotFound,
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

    fn fetch_all(&self) -> Result<Vec<Pokemon>, FetchAllError> {
        if self.error {
            return Err(FetchAllError::Unknown);
        }

        let mut pokemons = match self.pokemons.lock() {
            Ok(lock) => lock.to_vec(),
            Err(_) => return Err(FetchAllError::Unknown),
        };

        pokemons.sort_by(|a, b| a.number.cmp(&b.number));

        Ok(pokemons)
    }

    fn fetch_one(&self, number: PokemonNumber) -> Result<Pokemon, FetchOneError> {
        if self.error {
            return Err(FetchOneError::Unknown);
        }

        let pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(FetchOneError::Unknown),
        };

        match pokemons.iter().find(|p| p.number == number) {
            Some(pokemon) => Ok(pokemon.clone()),
            None => Err(FetchOneError::NotFound),
        }
    }

    fn delete(&self, number: PokemonNumber) -> Result<(), DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }

        let mut pokemons = match self.pokemons.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(DeleteError::Unknown),
        };

        let index = match pokemons.iter().position(|p| p.number == number) {
            Some(idx) => idx,
            None => return Err(DeleteError::NotFound),
        };

        pokemons.remove(index);

        Ok(())
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

    pub fn new_and_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }
}
