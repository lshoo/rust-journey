mod json_schema;

// pub use json_schema::*;  // Can't do this

use proc_macro::TokenStream;

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);

    TokenStream::default()
}
