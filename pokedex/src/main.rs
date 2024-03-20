use std::sync::Arc;

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use pokedex::{api, cli, repositories::pokemon::InMemoryRepository};

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::new("cli").long("cli").help("Run in CLI mode."))
        .get_matches();

    match matches.get_one::<String>("cli") {
        Some(_) => api::serve("localhost:8080", repo),
        _ => cli::create_pokemon::run(repo),
    }
}
