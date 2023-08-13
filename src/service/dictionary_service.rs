use crate::service::env_service::EnvService;

pub struct DictionaryService;

impl DictionaryService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn word_exists(&self, word: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let env_service = EnvService::new();

        let dictionary_uri = env_service.env_data.get("dictionary_uri");
        let dictionary_uri = match dictionary_uri {
            Some(value) => { value }
            None => { return Err("No dictionary_uri in .env".into()); }
        }.as_str();

        let client = reqwest::Client::new();
        let url = format!("{}/{}", dictionary_uri, word);
        let res = client
            .get(url)
            .send()
            .await?;

        Ok(res.status() != reqwest::StatusCode::NOT_FOUND)
    }
}