use std::{
    error::Error,
    fmt::{self, Formatter, Display},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, sqlx::Type)]
#[serde(crate = "rocket::serde")]
#[sqlx(type_name = "config_type")]
#[sqlx(rename_all = "lowercase")]
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
    type Err = Box<dyn Error + Send + Sync>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "value" => Ok(ConfigType::Value),
            "object" => Ok(ConfigType::Object),
            "array" => Ok(ConfigType::Array),
            _ => Err(format!("Unable to parse {}", input).into()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, sqlx::Type)]
#[serde(crate = "rocket::serde")]
#[sqlx(type_name = "value_type")]
#[sqlx(rename_all = "lowercase")]
pub enum ValueType {
    String,
    Boolean,
    Number,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ValueType::String => write!(f, "string"),
            ValueType::Boolean => write!(f, "boolean"),
            ValueType::Number => write!(f, "number"),
        }
    }
}

impl FromStr for ValueType {
    type Err = Box<dyn Error + Send + Sync>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "string" => Ok(ValueType::String),
            "boolean" => Ok(ValueType::Boolean),
            "number" => Ok(ValueType::Number),
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
    pub config_type: ConfigType,
    pub value_type: ValueType,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
