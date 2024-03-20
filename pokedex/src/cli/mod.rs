pub mod create_pokemon;

use std::sync::Arc;

use dialoguer::{theme::ColorfulTheme, Select};

use crate::repositories::pokemon::Repository;

pub fn run(repo: Arc<dyn Repository>) {
    loop {
        let choices = [
            "Fetch all Pokemons",
            "Fetch a Pokemon",
            "Create a Pokemon",
            "Delete a Pokemon",
            "Exit",
        ];

        let index = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Make your choice")
            .items(&choices)
            .default(0)
            .interact()
        {
            Ok(index) => index,
            _ => continue,
        };

        match index {
            2 => create_pokemon::run(repo.clone()),
            4 => break,
            _ => continue,
        }
    }
}
