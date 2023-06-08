use crate::{
    db::db::ConfigMonkeyDb,
    models::app::App,
    repos::apps_repo::{self, AppsRepoError},
};
use rocket_db_pools::Connection;

pub enum AppsServiceError {
    DuplicateSlug,
    Unknown,
}

impl AppsServiceError {
    pub fn code(&self) -> &str {
        match *self {
            AppsServiceError::DuplicateSlug => "duplicate_slug",
            AppsServiceError::Unknown => "unknown",
        }
    }
    pub fn message(&self) -> &str {
        match *self {
            AppsServiceError::DuplicateSlug => "An app with the same slug already exists",
            AppsServiceError::Unknown => "Unknown error",
        }
    }
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
