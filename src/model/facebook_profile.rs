use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FacebookProfile {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}