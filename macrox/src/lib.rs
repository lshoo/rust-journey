mod json_schema;

// pub use json_schema::*;  // Can't do this

use proc_macro::TokenStream;

use crate::json_schema::{get_string_literal, StructeTemplate};

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let filename = get_string_literal(input).unwrap();
    println!("{:#?}", filename);

    let result = StructeTemplate::render(&filename).unwrap();

    result.parse().unwrap()
}
