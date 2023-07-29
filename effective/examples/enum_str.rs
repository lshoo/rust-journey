use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Period {
    Hour,
    Day,
    Week,
}


fn main() {
    let p1: Period = serde_json::from_str("Hour").unwrap();
    println!("Period: {:?}", p1);
}