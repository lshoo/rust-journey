use std::sync::Arc;

use pokedex::{api, repositories::pokemon::InMemoryRepository};

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    api::serve("localhost:8080", repo);
}
