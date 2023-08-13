use std::collections::HashMap;

pub trait EnvReader {
    fn read(&self, path: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>;
}