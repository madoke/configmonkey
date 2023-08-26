use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum ConfigType {
    Value,
    Object,
    Array,
}

impl Display for ConfigType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ConfigType::Value => write!(f, "value"),
            ConfigType::Object => write!(f, "object"),
            ConfigType::Array => write!(f, "array"),
        }
    }
}

impl FromStr for ConfigType {
    type Err = Box<dyn Error>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "value" => Ok(ConfigType::Value),
            "object" => Ok(ConfigType::Object),
            "array" => Ok(ConfigType::Array),
            _ => Err(format!("Unable to parse {}", input).into()),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub id: String,
    pub key: String,
    pub version: i32,
    pub r#type: ConfigType,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
