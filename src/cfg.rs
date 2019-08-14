use std::fs::File;
use std::io::prelude::*;
use failure::Error;
use crate::failures::CustomError;
use config::{ConfigError, Config, File as CfgFile, Environment, FileFormat};
use clap::{App, Arg, SubCommand};
use serde::Deserialize;

#[derive(Debug,  Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub db: DbConfig,
    pub storage: StorageConfig,
    pub api: ApiConfig,
    pub secret: SecretConfig,
    pub client: ClientConfig,
    pub job: JobConfig
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub address: String,
    pub port: u16
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub valsys: String,
    pub sfa:  String
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub upload: String,
    pub url: String,
    pub log: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub path: String,
    pub version: String
}

#[derive(Debug, Deserialize)]
pub struct SecretConfig {
    pub secret: String,
    pub expired: String
}

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub base_url: String,
    pub path: String
}

#[derive(Debug, Deserialize)]
pub struct JobConfig {
    base_url: String
}

fn init_config(path: &str) -> Result<Config, Error> {
    let mut config = Config::default();
    let content: String = {
        let mut file = File::open(path)
            .map_err(|e| Error::from(CustomError::new(&format!("Could not open file {}: {}", path, e))))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| Error::from(CustomError::new(&format!("Could not read from file {}: {}", path, e))))?;
        content
    };
    config.merge(CfgFile::from_str(content.as_ref(), FileFormat::Toml))?;

    Ok(config)
}

pub fn app_config() -> Result<AppConfig, Error> {
    let matches = App::new("New Scorecard")
        .author("Angga Cipiet")
        .version("1.0.0")
        .about("New Scorecard")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .default_value("./config.toml")
            .help("Sets config file (TOML format)")
            .takes_value(true))
        .get_matches();

    //let cwd = ::std::env::current_dir()?;
    //let default_config = format!("{}/Settings.toml", cwd.display());
    //let config_file = matches.value_of("config").unwrap_or(&default_config);
    let config_file = matches.value_of("config").expect("invalid config value");

    println!("Using config file: {}", config_file);

    let config = init_config(&config_file)?;
    Ok(AppConfig {
        server: ServerConfig {
            name: config.get_str("server.name")?,
            address: config.get_str("server.address")?,
            port: config.get_int("server.port")? as u16,
        },
        db: DbConfig {
            valsys: config.get_str("database.valsys")?,
            sfa:  config.get_str("database.sfa")?,
        },
        storage: StorageConfig {
            upload: config.get_str("storage.upload")?,
            url: config.get_str("storage.url")?, 
            log: config.get_str("storage.log")?, 
        },
        api: ApiConfig {
            path: config.get_str("api.path")?,
            version: config.get_str("api.version")?,
        },
        secret: SecretConfig {
            secret: config.get_str("token.secret")?,
            expired: config.get_str("token.expired")?,
        },
        client: ClientConfig {
            base_url: config.get_str("client.base_url")?,
            path:  config.get_str("client.path")?,
        },
        job: JobConfig {
            base_url: config.get_str("icc.base_url")?,
        }
    })
}