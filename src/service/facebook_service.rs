use crate::model::facebook_profile::FacebookProfile;
use crate::model::user::User;
use crate::service::env_service::EnvService;

pub struct FacebookService;

impl FacebookService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_facebook_access_token(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        let env_service = EnvService::new();

        let client_id = env_service.env_data.get("client_id");
        let client_id = match client_id {
            Some(value) => { value }
            None => { return Err("No client_id in .env".into()); }
        }.as_str();

        let client_secret = env_service.env_data.get("client_secret");
        let client_secret = match client_secret {
            Some(value) => { value }
            None => { return Err("No client_secret in .env".into()); }
        }.as_str();

        let redirect_uri = env_service.env_data.get("redirect_uri");
        let redirect_uri = match redirect_uri {
            Some(value) => { value }
            None => { return Err("No redirect_uri in .env".into()); }
        }.as_str();

        let params = [
            ("client_id", client_id),
            ("redirect_uri", redirect_uri),
            ("client_secret", client_secret),
            ("code", code),
        ];

        let client = reqwest::Client::new();
        let res = client
            .get("https://graph.facebook.com/v17.0/oauth/access_token")
            .query(&params)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(res.get("access_token").unwrap().to_string().replace("\"", ""))
    }

    pub async fn get_facebook_profile(&self, access_token: &str) -> Result<FacebookProfile, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://graph.facebook.com/v17.0/me")
            .header("Authorization", format!("Bearer {}", &access_token))
            .send()
            .await?;

        let body = res.text().await?;
        let profile = serde_json::from_str(&body)?;
        Ok(profile)
    }
}

impl User {
    pub fn from_facebook_profile(profile: FacebookProfile) -> Self {
        Self {
            id: profile.id.clone(),
            name: profile.name.clone(),
            email: profile.email.unwrap_or(String::from("")).clone(),
            photo: "".to_string(),
            provider: "".to_string(),
        }
    }
}