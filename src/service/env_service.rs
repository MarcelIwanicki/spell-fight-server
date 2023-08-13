use std::collections::HashMap;

use crate::env::env_reader_factory::EnvReaderFactory;
use crate::env::file_type::FileType;

const ENV_PATH: &str = ".env";

pub struct EnvService {
    pub env_data: HashMap<String, String>,
}

impl EnvService {
    pub fn new() -> Self {
        let file_type = FileType::Json;
        let env_reader = EnvReaderFactory::create_env_reader(file_type);
        let data = env_reader.read(ENV_PATH);

        Self {
            env_data: data.unwrap_or(HashMap::new())
        }
    }
}