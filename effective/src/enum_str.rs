use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Period {
    Hour {},
    Day {},
    Week {},
}
