use crate::{
    db::db::ConfigMonkeyDb,
    models::config::Config,
    repos::configs_repo::{self, ConfigsRepoError},
};
use rocket_db_pools::Connection;

pub enum ConfigsServiceError {
    Unknown,
}

impl ConfigsServiceError {
    pub fn code(&self) -> &str {
        match *self {
            ConfigsServiceError::Unknown => "unknown",
        }
    }
    pub fn message(&self) -> &str {
        match *self {
            ConfigsServiceError::Unknown => "Unknown error",
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
    let result = configs_repo::create_config(db, app_slug, env_slug, config).await;
    match result {
        Ok(created_env) => Ok(created_env),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::Unknown => Err(ConfigsServiceError::Unknown),
        },
    }
}
