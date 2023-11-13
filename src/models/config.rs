use std::fmt::{self, Display};

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum ConfigValue {
    String(String),
    Boolean(bool),
    Float(f64),
    Integer(i64),
}

impl Display for ConfigValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigValue::String(s) => write!(f, "{}", s),
            ConfigValue::Boolean(b) => write!(f, "{}", b),
            ConfigValue::Float(n) => write!(f, "{}", n),
            ConfigValue::Integer(n) => write!(f, "{}", n),
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
