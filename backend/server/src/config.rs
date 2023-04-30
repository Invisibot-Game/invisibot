use std::{
    env::{self, VarError},
    io,
};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Dot env error")]
    DotEnvError(#[from] dotenv::Error),
    #[error("IO error")]
    IOError(#[from] io::Error),
    #[error("Environment variable error")]
    EnvVarError(#[from] VarError),
    #[error("Empty variable error")]
    VarEmpty(String),
    #[error("Invalid integer `{0}`")]
    InvalidInteger(String),
    #[error("Invalid boolean `{0}`")]
    InvalidBoolean(String),
}

type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub struct Config {
    pub websocket_port: u32,
    pub map_dir: String,
    pub development_mode: bool,
}

impl Config {
    pub fn new() -> ConfigResult<Config> {
        dotenv::dotenv()?;

        Ok(Config {
            websocket_port: load_env_num("WEBSOCKET_PORT")?,
            map_dir: load_env_str("MAP_DIR")?,
            development_mode: load_env_bool("DEVELOPMENT_MODE")?,
        })
    }
}

fn load_env_str(key: &str) -> ConfigResult<String> {
    let var = env::var(&key)?;

    if var.is_empty() {
        return Err(ConfigError::VarEmpty(key.to_string()));
    }

    Ok(var)
}

fn load_env_num(key: &str) -> ConfigResult<u32> {
    let var = load_env_str(key)?;
    var.parse().map_err(|_| ConfigError::InvalidInteger(var))
}

fn load_env_bool(key: &str) -> ConfigResult<bool> {
    let var = load_env_str(key)?;
    var.parse().map_err(|_| ConfigError::InvalidBoolean(var))
}
