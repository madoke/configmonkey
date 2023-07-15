use crate::{
    db::db::ConfigMonkeyDb,
    models::{app::App, list::List},
    repos::apps_repo::{self, AppsRepoError},
    shared::validators::{validate_name, validate_slug},
};
use rocket_db_pools::Connection;

pub enum AppsServiceError {
    DuplicateSlug,
    InvalidSlug,
    InvalidName,
    Unknown,
}

impl AppsServiceError {
    pub fn code(&self) -> &'static str {
        match *self {
            AppsServiceError::DuplicateSlug => "duplicate_slug",
            AppsServiceError::InvalidSlug => "invalid_slug",
            AppsServiceError::InvalidName => "invalid_name",
            AppsServiceError::Unknown => "unknown",
        }
    }
    pub fn message(&self) -> &'static str {
        match *self {
            AppsServiceError::DuplicateSlug => "An app with the same slug already exists",
            AppsServiceError::InvalidSlug => "The slug contains invalid characters. Only lowercase letters, numbers and dash (-) are allowed",
            AppsServiceError::InvalidName => "The name contains invalid characters. Only letters, numbers, spaces and underscore (_) are allowed",
            AppsServiceError::Unknown => "Unknown error",
        }
    }
}

const DEFAULT_LIMIT: i32 = 10;
const DEFAULT_OFFSET: i32 = 0;

pub async fn get_apps(
    db: Connection<ConfigMonkeyDb>,
    limit_opt: Option<i32>,
    offset_opt: Option<i32>,
) -> Result<List<App>, AppsServiceError> {
    let limit = limit_opt.unwrap_or(DEFAULT_LIMIT);
    let offset = offset_opt.unwrap_or(DEFAULT_OFFSET);

    let result = apps_repo::get_apps(db, limit, offset).await;
    match result {
        Ok(apps) => {
            let count = apps.len() as i32;
            Ok(List {
                items: apps,
                count,
                limit,
                offset,
                next_offset: if count == limit {
                    Some(offset + limit)
                } else {
                    None
                },
                prev_offset: if offset == 0 {
                    None
                } else if offset - limit >= 0 {
                    Some(offset - limit)
                } else {
                    Some(0)
                },
            })
        }
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