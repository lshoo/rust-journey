
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Period {
    Hour {},
    Day {},
    Week {},
}