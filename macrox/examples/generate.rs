pub mod gnerated {
    use macrox::generate;

    generate!("macrox/fixtures/person.json");
}

// use std::collections::HashMap;

// use serde::{Deserialize, Serialize};

use gnerated::*;

// #[derive(Debug, Default, Serialize, Deserialize)]
// pub struct Schema {
//     pub title: Option<String>,
//     #[serde(rename = "type")]
//     pub ty: String,
//     pub properties: Option<HashMap<String, Schema>>,
// }

fn main() {
    // let schema: Schema = serde_json::from_str(include_str!("../fixtures/person.json")).unwrap();

    // println!("{:?}", schema);
    let person = Person::default();
    println!("{:#?}", person);
}
