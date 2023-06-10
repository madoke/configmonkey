use crate::{
    db::db::ConfigMonkeyDb,
    models::app::App,
    repos::apps_repo::{self, AppsRepoError},
};
use lazy_static::lazy_static;
use regex::Regex;
use rocket_db_pools::Connection;

pub enum AppsServiceError {
    DuplicateSlug,
    InvalidSlug,
    InvalidName,
    Unknown,
}

impl AppsServiceError {
    pub fn code(&self) -> &str {
        match *self {
            AppsServiceError::DuplicateSlug => "duplicate_slug",
            AppsServiceError::InvalidSlug => "invalid_slug",
            AppsServiceError::InvalidName => "invalid_name",
            AppsServiceError::Unknown => "unknown",
        }
    }
    pub fn message(&self) -> &str {
        match *self {
            AppsServiceError::DuplicateSlug => "An app with the same slug already exists",
            AppsServiceError::InvalidSlug => "The slug contains invalid characters. Only lowercase letters, numbers and dash (-) are allowed",
            AppsServiceError::InvalidName => "The name contains invalid characters. Only letters, numbers, spaces and underscore (_) are allowed",
            AppsServiceError::Unknown => "Unknown error",
        }
    }
}

fn validate_slug(slug: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-z0-9\-]+$").unwrap();
    }
    RE.is_match(slug)
}

fn validate_name(slug: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\w+(\s+\w+)*$").unwrap();
    }
    RE.is_match(slug)
}

pub async fn get_apps(db: Connection<ConfigMonkeyDb>) -> Result<Vec<App>, AppsServiceError> {
    let result = apps_repo::get_apps(db).await;
    match result {
        Ok(apps) => Ok(apps),
        Err(apps_repo_err) => match apps_repo_err {
            _ => Err(AppsServiceError::Unknown),
        },
    }
}

pub async fn create_app(
    db: Connection<ConfigMonkeyDb>,
    slug: &str,
    name: &str,
) -> Result<App, AppsServiceError> {
    let is_valid_slug = validate_slug(slug);
    if !is_valid_slug {
        return Err(AppsServiceError::InvalidSlug);
    }

    let is_valid_name = validate_name(name);
    if !is_valid_name {
        return Err(AppsServiceError::InvalidName);
    }

    let result = apps_repo::create_app(db, slug, name).await;
    match result {
        Ok(created_app) => Ok(created_app),
        Err(apps_repo_err) => match apps_repo_err {
            AppsRepoError::DuplicateSlug => Err(AppsServiceError::DuplicateSlug),
            AppsRepoError::Unknown => Err(AppsServiceError::Unknown),
        },
    }
}

pub async fn delete_app(
    db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<(), AppsServiceError> {
    let result = apps_repo::delete_app(db, slug).await;
    match result {
        Ok(()) => Ok(()),
        Err(apps_repo_err) => match apps_repo_err {
            _ => Err(AppsServiceError::Unknown),
        },
    }
}
