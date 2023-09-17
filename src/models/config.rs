use std::fmt::{self, Display};

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum ConfigValue {
    String(String),
    Boolean(bool),
    Number(f64),
}

impl Display for ConfigValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigValue::String(s) => write!(f, "{}", s),
            ConfigValue::Boolean(b) => write!(f, "{}", b),
            ConfigValue::Number(n) => write!(f, "{}", n),
        }
    }
}

impl ConfigValue {
    pub fn r#type(&self) -> &str {
        match self {
            ConfigValue::String(_) => "string",
            ConfigValue::Boolean(_) => "boolean",
            ConfigValue::Number(_) => "number",
        }
    }
}

#[derive(Debug)]
pub struct ConfigVersion {
    pub id: String,
    pub version: i32,
    pub value: ConfigValue,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Config {
    pub id: String,
    pub key: String,
    pub created_at: DateTime<Utc>,
}
