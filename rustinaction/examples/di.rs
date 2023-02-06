//! https://medium.com/geekculture/dependency-injection-in-rust-3822bf689888

use std::fmt::Debug;

fn log(ts: Vec<Box<dyn Debug>>) {
    ts.into_iter().for_each(|t| println!("{t:?}"));
}

fn main() {
    log(vec![
        Box::new(false),
        Box::new(2),
        Box::new("james".to_string()),
    ]);
}
