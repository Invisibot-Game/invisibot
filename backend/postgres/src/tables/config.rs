use crate::postgres_error::{PostgresError, PostgresResult};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Config {
    pub config_name: String,
    pub config_value: String,
    pub config_type: String,
}

impl Config {
    pub fn to_config_entry(&self) -> PostgresResult<ConfigEntry> {
        let config_value = match self.config_type.as_str() {
            "u32" => {
                let num = u32::from_str_radix(&self.config_value, 10).or_else(|err| {
                    Err(PostgresError::InvalidConfigType(
                        self.config_value.clone(),
                        self.config_name.clone(),
                        self.config_type.clone(),
                        err.to_string(),
                    ))
                })?;
                ConfigValue::U32(num)
            }
            t => {
                todo!("Config value type '{t}' not implemented!")
            }
        };

        Ok(ConfigEntry {
            key: self.config_name.clone(),
            value: config_value,
        })
    }
}

pub struct ConfigEntry {
    pub key: String,
    pub value: ConfigValue,
}

pub enum ConfigValue {
    U32(u32),
}
