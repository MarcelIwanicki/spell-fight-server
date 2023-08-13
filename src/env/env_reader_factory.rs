use crate::env::file_type::FileType;
use crate::env::json_reader::JsonEnvReader;
use crate::env::reader::EnvReader;

pub struct EnvReaderFactory;
impl EnvReaderFactory {
    pub fn create_env_reader(file_type: FileType) -> Box<dyn EnvReader> {
        match file_type {
            FileType::Json => Box::new(JsonEnvReader),
        }
    }
}