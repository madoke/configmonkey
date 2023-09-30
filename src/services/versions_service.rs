use crate::{
    db::db::ConfigMonkeyDb,
    models::{
        config::{ConfigValue, ConfigVersion},
        list::List,
    },
    repos::{
        configs_repo::{self, ConfigsRepoError},
        domains_repo::{self, DomainsRepoError},
        versions_repo::{self, VersionsRepoError},
    },
};

use rocket::error;
use rocket_db_pools::Connection;

pub enum VersionsServiceError {
    Unknown,
    DomainNotFound,
    ConfigNotFound,
}

impl VersionsServiceError {
    pub fn code(&self) -> &'static str {
        match *self {
            VersionsServiceError::ConfigNotFound => "config_not_found",
            VersionsServiceError::DomainNotFound => "domain_not_found",
            VersionsServiceError::Unknown => "unknown_error",
        }
    }
    pub fn message(&self) -> &'static str {
        match *self {
            VersionsServiceError::ConfigNotFound => "Config not found",
            VersionsServiceError::DomainNotFound => "Domain not found",
            VersionsServiceError::Unknown => "Unknown error",
        }
    }
}

const DEFAULT_LIMIT: i32 = 10;
const DEFAULT_OFFSET: i32 = 0;

pub async fn create_version(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    config_value: ConfigValue,
) -> Result<ConfigVersion, VersionsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(VersionsServiceError::DomainNotFound),
            _ => {
                error!(
                    "[create_version] Error fetching domains: {:?}",
                    get_domain_error
                );
                return Err(VersionsServiceError::Unknown);
            }
        }
    }
    // Get Config
    let config_result =
        configs_repo::get_config(&mut *db, domain_result.unwrap().id.as_str(), key).await;
    if let Err(get_config_error) = config_result {
        match get_config_error {
            ConfigsRepoError::NotFound => return Err(VersionsServiceError::ConfigNotFound),
            _ => {
                error!(
                    "[create_version] Error fetching config: {:?}",
                    get_config_error
                );
                return Err(VersionsServiceError::Unknown);
            }
        }
    }

    let result =
        versions_repo::create_version(&mut *db, config_result.unwrap().id.as_str(), config_value)
            .await;
    match result {
        Ok(version) => Ok(version),
        Err(configs_repo_err) => match configs_repo_err {
            VersionsRepoError::NotFound => Err(VersionsServiceError::ConfigNotFound),
            _ => Err(VersionsServiceError::Unknown),
        },
    }
}

pub async fn get_versions(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    limit_opt: Option<i32>,
    offset_opt: Option<i32>,
) -> Result<List<ConfigVersion>, VersionsServiceError> {
    // Get domain
    let domain_result = domains_repo::get_domain_by_slug(&mut *db, domain_slug).await;
    if let Err(get_domain_error) = domain_result {
        match get_domain_error {
            DomainsRepoError::NotFound => return Err(VersionsServiceError::DomainNotFound),
            _ => {
                error!(
                    "[get_versions] Error fetching domains: {:?}",
                    get_domain_error
                );
                return Err(VersionsServiceError::Unknown);
            }
        }
    }
    // Get Config
    let config_result =
        configs_repo::get_config(&mut *db, domain_result.unwrap().id.as_str(), key).await;
    if let Err(get_config_error) = config_result {
        match get_config_error {
            ConfigsRepoError::NotFound => return Err(VersionsServiceError::ConfigNotFound),
            _ => {
                error!(
                    "[get_versions] Error fetching config: {:?}",
                    get_config_error
                );
                return Err(VersionsServiceError::Unknown);
            }
        }
    }

    // Get versions
    let limit = limit_opt.unwrap_or(DEFAULT_LIMIT);
    let offset = offset_opt.unwrap_or(DEFAULT_OFFSET);

    let result =
        versions_repo::get_versions(&mut *db, config_result.unwrap().id.as_str(), limit, offset)
            .await;
    match result {
        Ok(versions) => Ok(List::from_items(versions, limit, offset)),
        Err(configs_repo_err) => match configs_repo_err {
            VersionsRepoError::NotFound => Err(VersionsServiceError::ConfigNotFound),
            _ => Err(VersionsServiceError::Unknown),
        },
    }
}
