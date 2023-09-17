use crate::{
    db::db::ConfigMonkeyDb,
    models::{
        config::{Config, ConfigValue, ConfigVersion},
        list::List,
    },
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
    ConfigNotFound,
    AlreadyExists,
}

impl ConfigsServiceError {
    pub fn code(&self) -> &'static str {
        match *self {
            ConfigsServiceError::AlreadyExists => "already_exists",
            ConfigsServiceError::ConfigNotFound => "config_not_found",
            ConfigsServiceError::DomainNotFound => "domain_not_found",
            ConfigsServiceError::Unknown => "unknown_error",
        }
    }
    pub fn message(&self) -> &'static str {
        match *self {
            ConfigsServiceError::AlreadyExists => "Config already exists",
            ConfigsServiceError::ConfigNotFound => "Config not found",
            ConfigsServiceError::DomainNotFound => "Domain not found",
            ConfigsServiceError::Unknown => "Unknown error",
        }
    }
}

const DEFAULT_LIMIT: i32 = 10;
const DEFAULT_OFFSET: i32 = 0;

pub async fn create_config(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
) -> Result<Config, ConfigsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(ConfigsServiceError::DomainNotFound),
            _ => {
                error!(
                    "[create_config] Error fetching domains: {:?}",
                    get_domain_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }

    // Create config
    let result =
        configs_repo::create_config(&mut *db, domain_result.unwrap().id.as_str(), key).await;
    match result {
        Ok(created_config) => Ok(created_config),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::AlreadyExists => Err(ConfigsServiceError::AlreadyExists),
            ConfigsRepoError::NotFound => Err(ConfigsServiceError::DomainNotFound),
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}

pub async fn get_config(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
) -> Result<Config, ConfigsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(ConfigsServiceError::DomainNotFound),
            _ => {
                error!(
                    "[get_config] Error fetching domains: {:?}",
                    get_domain_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }

    // Get config
    let result = configs_repo::get_config(&mut *db, domain_result.unwrap().id.as_str(), key).await;
    match result {
        Ok(config) => Ok(config),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::NotFound => Err(ConfigsServiceError::DomainNotFound),
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}

pub async fn get_configs(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    limit_opt: Option<i32>,
    offset_opt: Option<i32>,
) -> Result<List<Config>, ConfigsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(ConfigsServiceError::DomainNotFound),
            _ => {
                error!(
                    "[get_configs] Error fetching domains: {:?}",
                    get_domain_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }

    // Get configs
    let limit = limit_opt.unwrap_or(DEFAULT_LIMIT);
    let offset = offset_opt.unwrap_or(DEFAULT_OFFSET);
    let result =
        configs_repo::get_configs(&mut *db, domain_result.unwrap().id.as_str(), limit, offset)
            .await;
    match result {
        Ok(configs) => Ok(List::from_items(configs, limit, offset)),
        Err(configs_repo_err) => match configs_repo_err {
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}

pub async fn delete_config(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
) -> Result<(), ConfigsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(ConfigsServiceError::DomainNotFound),
            _ => {
                error!(
                    "[delete_config] Error fetching domains: {:?}",
                    get_domain_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }
    // Get Config
    let config_result =
        configs_repo::get_config(&mut *db, domain_result.unwrap().id.as_str(), key).await;
    if let Err(get_config_error) = config_result {
        match get_config_error {
            ConfigsRepoError::NotFound => return Err(ConfigsServiceError::ConfigNotFound),
            _ => {
                error!(
                    "[delete_config] Error fetching config: {:?}",
                    get_config_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }
    let result = configs_repo::delete_config(&mut *db, config_result.unwrap().id.as_str()).await;
    match result {
        Ok(()) => Ok(()),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::NotFound => Err(ConfigsServiceError::ConfigNotFound),
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}

pub async fn create_version(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    config_value: ConfigValue,
) -> Result<ConfigVersion, ConfigsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(ConfigsServiceError::DomainNotFound),
            _ => {
                error!(
                    "[create_version] Error fetching domains: {:?}",
                    get_domain_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }
    // Get Config
    let config_result =
        configs_repo::get_config(&mut *db, domain_result.unwrap().id.as_str(), key).await;
    if let Err(get_config_error) = config_result {
        match get_config_error {
            ConfigsRepoError::NotFound => return Err(ConfigsServiceError::ConfigNotFound),
            _ => {
                error!(
                    "[create_version] Error fetching config: {:?}",
                    get_config_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }

    let result =
        configs_repo::create_version(&mut *db, config_result.unwrap().id.as_str(), config_value)
            .await;
    match result {
        Ok(version) => Ok(version),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::NotFound => Err(ConfigsServiceError::ConfigNotFound),
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}

pub async fn get_versions(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    limit_opt: Option<i32>,
    offset_opt: Option<i32>,
) -> Result<List<ConfigVersion>, ConfigsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(ConfigsServiceError::DomainNotFound),
            _ => {
                error!(
                    "[get_versions] Error fetching domains: {:?}",
                    get_domain_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }
    // Get Config
    let config_result =
        configs_repo::get_config(&mut *db, domain_result.unwrap().id.as_str(), key).await;
    if let Err(get_config_error) = config_result {
        match get_config_error {
            ConfigsRepoError::NotFound => return Err(ConfigsServiceError::ConfigNotFound),
            _ => {
                error!(
                    "[get_versions] Error fetching config: {:?}",
                    get_config_error
                );
                return Err(ConfigsServiceError::Unknown);
            }
        }
    }

    // Get versions
    let limit = limit_opt.unwrap_or(DEFAULT_LIMIT);
    let offset = offset_opt.unwrap_or(DEFAULT_OFFSET);

    let result =
        configs_repo::get_versions(&mut *db, config_result.unwrap().id.as_str(), limit, offset)
            .await;
    match result {
        Ok(versions) => Ok(List::from_items(versions, limit, offset)),
        Err(configs_repo_err) => match configs_repo_err {
            ConfigsRepoError::NotFound => Err(ConfigsServiceError::ConfigNotFound),
            _ => Err(ConfigsServiceError::Unknown),
        },
    }
}
