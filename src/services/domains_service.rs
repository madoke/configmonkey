use crate::{
    db::db::ConfigMonkeyDb,
    models::{domain::Domain, list::List},
    repos::domains_repo::{self, DomainsRepoError},
    shared::validators::validate_slug,
};
use rocket_db_pools::Connection;

pub enum DomainsServiceError {
    DuplicateSlug,
    InvalidSlug,
    NotEmpty,
    NotFound,
    Unknown,
}

impl DomainsServiceError {
    pub fn code(&self) -> &'static str {
        match *self {
            DomainsServiceError::DuplicateSlug => "duplicate_slug",
            DomainsServiceError::InvalidSlug => "invalid_slug",
            DomainsServiceError::NotEmpty => "not_empty",
            DomainsServiceError::NotFound => "not_found",
            DomainsServiceError::Unknown => "unknown",
        }
    }
    pub fn message(&self) -> &'static str {
        match *self {
            DomainsServiceError::DuplicateSlug => "A domain with the same slug already exists",
            DomainsServiceError::InvalidSlug => "The slug contains invalid characters. Only letters, numbers, dash (-) and underscore (_) are allowed",
            DomainsServiceError::NotEmpty => "The domain could not be deleted because there are existing configs",
            DomainsServiceError::NotFound => "Domain not found",
            DomainsServiceError::Unknown => "Unknown error",
        }
    }
}

const DEFAULT_LIMIT: i32 = 10;
const DEFAULT_OFFSET: i32 = 0;

pub async fn create_domain(
    db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<Domain, DomainsServiceError> {
    let is_valid_slug = validate_slug(slug);
    if !is_valid_slug {
        return Err(DomainsServiceError::InvalidSlug);
    }

    let result = domains_repo::create_domain(db, slug).await;
    match result {
        Ok(domain) => Ok(domain),
        Err(err) => match err {
            DomainsRepoError::DuplicateSlug => Err(DomainsServiceError::DuplicateSlug),
            _ => Err(DomainsServiceError::Unknown),
        },
    }
}

pub async fn get_domains(
    db: Connection<ConfigMonkeyDb>,
    limit_opt: Option<i32>,
    offset_opt: Option<i32>,
) -> Result<List<Domain>, DomainsServiceError> {
    let limit = limit_opt.unwrap_or(DEFAULT_LIMIT);
    let offset = offset_opt.unwrap_or(DEFAULT_OFFSET);

    let result = domains_repo::get_domains(db, limit, offset).await;
    match result {
        Ok(domains) => Ok(List::from_items(domains, limit, offset)),
        Err(err) => match err {
            _ => Err(DomainsServiceError::Unknown),
        },
    }
}

pub async fn delete_domain(
    db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<(), DomainsServiceError> {
    let result = domains_repo::delete_domain(db, slug).await;
    match result {
        Ok(()) => Ok(()),
        Err(err) => match err {
            DomainsRepoError::NotFound => Err(DomainsServiceError::NotFound),
            DomainsRepoError::NotEmpty => Err(DomainsServiceError::NotEmpty),
            _ => Err(DomainsServiceError::Unknown),
        },
    }
}
