use crate::{
    db::db::ConfigMonkeyDb,
    models::config::Config,
    repos::configs_repo::{self, ConfigsRepoError},
};
use rocket::serde::{
    de::IgnoredAny,
    json::serde_json::{from_str, Error},
};
use rocket_db_pools::Connection;

pub enum ConfigsServiceError {
    Unknown,
    AppOrEnvNotFound,
    ConfigAlreadyExists,
    InvalidConfigFormat,
}

impl ConfigsServiceError {
    pub fn code(&self) -> &'static str {
        match *self {
            ConfigsServiceError::Unknown => "unknown",
            ConfigsServiceError::AppOrEnvNotFound => "app_or_env_not_found",
            ConfigsServiceError::ConfigAlreadyExists => "config_already_exists",
            ConfigsServiceError::InvalidConfigFormat => "invalid_config_format",
        }
    }
    pub fn message(&self) -> &'static str {
        match *self {
            ConfigsServiceError::Unknown => "Unknown error",
            ConfigsServiceError::AppOrEnvNotFound => "App or env not found",
            ConfigsServiceError::ConfigAlreadyExists => "Config already exists",
            ConfigsServiceError::InvalidConfigFormat => "Invalid config format. Check the payload",
        }
    }
}

pub async fn get_config(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
) -> Result<Config, ConfigsServiceError> {
    let result = configs_repo::get_config(db, app_slug, env_slug).await;
    match result {
        Ok(config) => Ok(config),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::AppOrEnvNotFound => Err(ConfigsServiceError::AppOrEnvNotFound),
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}

pub async fn create_config(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
    config: &str,
) -> Result<Config, ConfigsServiceError> {
    let parsed_value: Result<IgnoredAny, Error> = from_str(config);
    if parsed_value.is_err() {
        return Err(ConfigsServiceError::InvalidConfigFormat);
    }

    let result = configs_repo::create_config(db, app_slug, env_slug, config).await;
    match result {
        Ok(created_env) => Ok(created_env),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::InvalidConfigJson => Err(ConfigsServiceError::InvalidConfigFormat),
            ConfigsRepoError::ConfigAlreadyExists => Err(ConfigsServiceError::ConfigAlreadyExists),
            ConfigsRepoError::AppOrEnvNotFound => Err(ConfigsServiceError::AppOrEnvNotFound),
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}

pub async fn delete_config(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
) -> Result<(), ConfigsServiceError> {
    let result = configs_repo::delete_config(db, app_slug, env_slug).await;
    match result {
        Ok(()) => Ok(()),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::AppOrEnvNotFound => Err(ConfigsServiceError::AppOrEnvNotFound),
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}
