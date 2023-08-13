use serde::Deserialize;

#[derive(Deserialize)]
pub struct CallbackData {
    pub code: String,
}