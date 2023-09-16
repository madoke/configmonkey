use crate::{
    db::db::ConfigMonkeyDb,
    models::config::{Config, ConfigValue},
    repos::{
        configs_repo::{self, ConfigsRepoError},
        domains_repo::{self, DomainsRepoError},
    },
};

use rocket::error;
use rocket_db_pools::Connection;

pub enum ConfigsServiceError {
    Unknown,
    DomainNotFound,
}

impl ConfigsServiceError {
    pub fn code(&self) -> &'static str {
        match *self {
            ConfigsServiceError::DomainNotFound => "domain_not_found",
            ConfigsServiceError::Unknown => "unknown_error",
        }
    }
    pub fn message(&self) -> &'static str {
        match *self {
            ConfigsServiceError::DomainNotFound => "Domain not found",
            ConfigsServiceError::Unknown => "Unknown error",
        }
    }
}

pub async fn create_config(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    config_value: ConfigValue,
) -> Result<Config, ConfigsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(ConfigsServiceError::DomainNotFound),
            _ => {
                error!("Unknown error fetching domains {:?}", get_domain_error);
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }

    // Create config
    let result = configs_repo::create_config(
        &mut *db,
        domain_result.unwrap().id.as_str(),
        key,
        config_value,
    )
    .await;
    match result {
        Ok(created_config) => Ok(created_config),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::NotFound => Err(ConfigsServiceError::DomainNotFound),
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
