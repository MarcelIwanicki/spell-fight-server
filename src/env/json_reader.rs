use std::collections::HashMap;
use std::fs;

use crate::env::reader::EnvReader;

pub struct JsonEnvReader;

impl EnvReader for JsonEnvReader {
    fn read(&self, path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let env_vars: HashMap<String, String> = serde_json::from_str(&data)?;
        Ok(env_vars)
    }
}