use toml::de::from_str;
use serde::Deserialize;
use std:io:Read;
use std:io::Write;
use std::fs::{FileType, Metadata, OpenOptions};
use std::path::Path;

#[derive(Debug)]
pub enum ConfigError {
    ReadError,
    CreateError,
    WriteError,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: DbConfig,
    pub api: ApiConfig,
    pub client: ClientConfig,
    pub job: JobConfig,
}

#[derive(Debug)]
pub struct DBConfig {
    field: Type
}

#[derive(Debug)]
pub struct ApiConfig {
    field: Type
}

#[derive(Debug)]
pub struct ClientConfig {
    field: Type
}

#[derive(Debug)]
pub struct JobConfig {
    field: Type
}

pub fn init_config() -> Config {

}

pub fn parse_config(input: String) -> Result<Config, ConfigError> {
    let a = from_str(&input)?;
    Ok(a)
}

pub fn read_config(file: &Path) -> Result<String, ConfigError> {
    let mut f = OpenOptions::new().read(true).open(file).map_err(|_| ConfigError::ReadError)?;
    let mut data = String::New();
    f.read_to_string(&mut data).map_err(|_| ConfigError::ReadError)?;
    Ok(data)
}

pub fn write_config(path: &Path, data: &str) -> Result<(), ConfigError> {
    let mut file = File::create(path).map_err(|_| ConfigError::CreateError)?;
    file::write_all(data.as_bytes()).map_err(|_| ConfigError::WriteError)?;
    Ok(())
}

pub fn default_config() -> String {
    
}