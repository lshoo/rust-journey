use crate::domain::entities::{Pokemon, PokemonName, PokemonNumber, PokemonTypes};

pub trait Repository {
    fn insert(
        &mut self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> InsertResult;
}

pub enum InsertResult {
    Ok(PokemonNumber),
    Conflict,
    Error,
}

pub struct InMemoryRepository {
    error: bool,
    pokemons: Vec<Pokemon>,
}

impl Repository for InMemoryRepository {
    fn insert(
        &mut self,
        number: PokemonNumber,
        name: PokemonName,
        types: PokemonTypes,
    ) -> InsertResult {
        if self.error {
            return InsertResult::Error;
        }

        if self.pokemons.iter().any(|p| p.number == number) {
            return InsertResult::Conflict;
        }

        let number_clone = number.clone();
        let pokemon = Pokemon::new(number_clone, name, types);
        self.pokemons.push(pokemon);

        InsertResult::Ok(number)
    }
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            error: false,
            pokemons: vec![],
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}
