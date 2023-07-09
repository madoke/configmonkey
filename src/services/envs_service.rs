use crate::{
    db::db::ConfigMonkeyDb,
    models::{env::Env, list::List},
    repos::envs_repo::{self, EnvsRepoError},
};
use lazy_static::lazy_static;
use regex::Regex;
use rocket_db_pools::Connection;

pub enum EnvsServiceError {
    DuplicateSlug,
    InvalidSlug,
    InvalidName,
    Unknown,
}

impl EnvsServiceError {
    pub fn code(&self) -> &'static str {
        match *self {
            EnvsServiceError::DuplicateSlug => "duplicate_slug",
            EnvsServiceError::InvalidSlug => "invalid_slug",
            EnvsServiceError::InvalidName => "invalid_name",
            EnvsServiceError::Unknown => "unknown",
        }
    }
    pub fn message(&self) -> &'static str {
        match *self {
            EnvsServiceError::DuplicateSlug => "An app with the same slug already exists",
            EnvsServiceError::InvalidSlug => "The slug contains invalid characters. Only lowercase letters, numbers and dash (-) are allowed",
            EnvsServiceError::InvalidName => "The name contains invalid characters. Only letters, numbers, spaces and underscore (_) are allowed",
            EnvsServiceError::Unknown => "Unknown error",
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

const DEFAULT_LIMIT: i32 = 10;
const DEFAULT_OFFSET: i32 = 0;

pub async fn get_envs(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    limit_opt: Option<i32>,
    offset_opt: Option<i32>,
) -> Result<List<Env>, EnvsServiceError> {
    let limit = limit_opt.unwrap_or(DEFAULT_LIMIT);
    let offset = offset_opt.unwrap_or(DEFAULT_OFFSET);

    let result = envs_repo::get_envs(db, app_slug, limit, offset).await;
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
        Err(envs_repo_err) => match envs_repo_err {
            _ => Err(EnvsServiceError::Unknown),
        },
    }
}

pub async fn create_env(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    slug: &str,
    name: &str,
) -> Result<Env, EnvsServiceError> {
    let is_valid_slug = validate_slug(slug);
    if !is_valid_slug {
        return Err(EnvsServiceError::InvalidSlug);
    }

    let is_valid_name = validate_name(name);
    if !is_valid_name {
        return Err(EnvsServiceError::InvalidName);
    }

    let result = envs_repo::create_env(db, app_slug, slug, name).await;
    match result {
        Ok(created_env) => Ok(created_env),
        Err(envs_repo_err) => match envs_repo_err {
            EnvsRepoError::DuplicateSlug => Err(EnvsServiceError::DuplicateSlug),
            EnvsRepoError::Unknown => Err(EnvsServiceError::Unknown),
        },
    }
}

pub async fn delete_env(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    slug: &str,
) -> Result<(), EnvsServiceError> {
    let result = envs_repo::delete_env(db, app_slug, slug).await;
    match result {
        Ok(()) => Ok(()),
        Err(envs_repo_err) => match envs_repo_err {
            _ => Err(EnvsServiceError::Unknown),
        },
    }
}
