use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub provider: String,
}