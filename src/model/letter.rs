use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct Letter {
    pub letter: char,
    pub value: u32,
}