use std::str::FromStr;

use crate::{
    db::db::ConfigMonkeyDb,
    models::config::{Config, ConfigType, ValueType},
    repos::configs_repo::{self},
};
use rocket_db_pools::Connection;

pub enum ConfigsServiceError {
    Unknown,
    UnknownValueType,
    UnknownConfigType,
    DomainNotFound,
}

impl ConfigsServiceError {
    pub fn code(&self) -> &'static str {
        match *self {
            ConfigsServiceError::UnknownValueType => "unknown_value_type",
            ConfigsServiceError::UnknownConfigType => "unknown_config_type",
            ConfigsServiceError::DomainNotFound => "domain_not_found",
            ConfigsServiceError::Unknown => "unknown_error",
        }
    }
    pub fn message(&self) -> &'static str {
        match *self {
            ConfigsServiceError::UnknownValueType => "Unknown value type",
            ConfigsServiceError::UnknownConfigType => "Unknown config type",
            ConfigsServiceError::DomainNotFound => "Domain not found",
            ConfigsServiceError::Unknown => "Unknown error",
        }
    }
}

pub async fn create_config(
    db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    config_type: &str,
    value_type: &str,
    value: &str,
) -> Result<Config, ConfigsServiceError> {
    // Parse input
    let value_type_parsed = ValueType::from_str(value_type);
    if let Err(_parse_error) = value_type_parsed {
        return Err(ConfigsServiceError::UnknownValueType);
    }
    let config_type_parsed = ConfigType::from_str(config_type);
    if let Err(_parse_error) = config_type_parsed {
        return Err(ConfigsServiceError::UnknownConfigType);
    }
    // Create config
    let result = configs_repo::create_config(
        db,
        domain_slug,
        key,
        config_type_parsed.unwrap(),
        value_type_parsed.unwrap(),
        value,
    )
    .await;
    match result {
        Ok(created_env) => Ok(created_env),
        Err(configs_repo_err) => match configs_repo_err {
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}

// pub async fn get_config(
//     db: Connection<ConfigMonkeyDb>,
//     app_slug: &str,
//     env_slug: &str,
// ) -> Result<Config, ConfigsServiceError> {
//     let result = configs_repo::get_config(db, app_slug, env_slug).await;
//     match result {
//         Ok(config) => Ok(config),
//         Err(configs_repo_err) => match configs_repo_err {
//             ConfigsRepoError::AppOrEnvNotFound => Err(ConfigsServiceError::AppOrEnvNotFound),
//             _ => Err(ConfigsServiceError::Unknown),
//         },
//     }
// }

// pub async fn delete_config(
//     db: Connection<ConfigMonkeyDb>,
//     app_slug: &str,
//     env_slug: &str,
// ) -> Result<(), ConfigsServiceError> {
//     let result = configs_repo::delete_config(db, app_slug, env_slug).await;
//     match result {
//         Ok(()) => Ok(()),
//         Err(configs_repo_err) => match configs_repo_err {
//             ConfigsRepoError::AppOrEnvNotFound => Err(ConfigsServiceError::AppOrEnvNotFound),
//             _ => Err(ConfigsServiceError::Unknown),
//         },
//     }
// }
