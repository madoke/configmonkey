use crate::{db::db::ConfigMonkeyDb, models::domain::Domain};
use chrono::{DateTime, Utc};
use rocket::{error, log::private::debug};
use rocket_db_pools::{
    sqlx::{self, types::Uuid},
    Connection,
};
use sqlx::{Error, Postgres, pool::PoolConnection};
use std::borrow::Cow;

#[derive(Debug)]
pub enum DomainsRepoError {
    Unknown,
    DuplicateSlug,
    NotFound,
    NotEmpty,
}

#[derive(sqlx::FromRow, Debug)]
struct DomainEntity {
    pub id: Uuid,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

fn map_sqlx_error(error: Error) -> DomainsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            Some(Cow::Borrowed("23505")) => DomainsRepoError::DuplicateSlug,
            Some(Cow::Borrowed("23503")) => DomainsRepoError::NotEmpty,
            _ => DomainsRepoError::Unknown,
        },
        Error::RowNotFound => DomainsRepoError::NotFound,
        _ => DomainsRepoError::Unknown,
    }
}

/// Create a new domain
pub async fn create_domain(
    mut db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<Domain, DomainsRepoError> {
    let result = sqlx::query_as::<_, DomainEntity>(
        "insert into domains(slug) values($1) returning id, slug, created_at",
    )
    .bind(slug)
    .fetch_one(&mut *db)
    .await;

    match result {
        Ok(domain) => {
            debug!("Successfully created domain: {:?}", domain);
            Ok(Domain {
                id: domain.id.to_string(),
                slug: domain.slug,
                created_at: domain.created_at,
            })
        }
        Err(err) => {
            error!(
                "Error creating domain with slug: {}. Error: {:?}",
                slug, err
            );
            Err(map_sqlx_error(err))
        }
    }
}

/// Retrieve the list of domains
pub async fn get_domains(
    mut db: Connection<ConfigMonkeyDb>,
    limit: i32,
    offset: i32,
) -> Result<Vec<Domain>, DomainsRepoError> {
    let domains_result = sqlx::query_as::<_, DomainEntity>(
        "select id, slug, created_at from domains limit $1 offset $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *db)
    .await;

    match domains_result {
        Ok(domains) => {
            debug!("Successfully retrieved domains: {:?}", domains);
            let mut result = vec![];
            for domain in domains {
                result.push(Domain {
                    id: domain.id.to_string(),
                    slug: domain.slug,
                    created_at: domain.created_at,
                })
            }
            Ok(result)
        }
        Err(err) => {
            error!("Error retrieving domains. Error: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}

/// Retrieve domain by slug
pub async fn get_domain_by_slug(
    db: &mut PoolConnection<Postgres>,
    domain_slug: &str,
) -> Result<Domain, DomainsRepoError> {
    let domain_result = sqlx::query_as::<_, DomainEntity>(
        "select id, slug, created_at from domains where slug = $1",
    )
    .bind(domain_slug)
    .fetch_one( db)
    .await;
    match domain_result {
        Ok(domain) => {
            debug!("Successfully retrieved domain: {:?}", domain);
            Ok(Domain {
                id: domain.id.to_string(),
                slug: domain.slug,
                created_at: domain.created_at,
            })
        }
        Err(err) => {
            error!("Error retrieving domain. Error: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}

pub async fn delete_domain(
    mut db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<(), DomainsRepoError> {
    let result = sqlx::query("delete from domains where slug = $1")
        .bind(slug)
        .execute(&mut *db)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("Domain {} not found", slug);
                return Err(DomainsRepoError::NotFound);
            }
            debug!("Successfully deleted domain with slug: {}", slug);
            Ok(())
        }
        Err(err) => {
            error!("Error deleting domain {}. Error: {:?}", slug, err);
            Err(map_sqlx_error(err))
        }
    }
}
