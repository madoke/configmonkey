use crate::{db::db::ConfigMonkeyDb, models::app::App};
use chrono::{DateTime, Utc};
use rocket::log::private::debug;
use rocket_db_pools::{
    sqlx::{self, types::Uuid},
    Connection,
};
use sqlx::Error;
use std::borrow::Cow;

pub enum AppsRepoError {
    DuplicateSlug,
    Unknown,
}

fn map_sqlx_error(error: Error) -> AppsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            Some(Cow::Borrowed("23505")) => AppsRepoError::DuplicateSlug,
            _ => AppsRepoError::Unknown,
        },
        _ => AppsRepoError::Unknown,
    }
}

#[derive(sqlx::FromRow, Debug)]
struct AppEntity {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_apps(
    mut db: Connection<ConfigMonkeyDb>,
    limit: i32,
    offset: i32,
) -> Result<Vec<App>, AppsRepoError> {
    let result = sqlx::query_as::<_, AppEntity>(
        "select id, slug, name, created_at, updated_at from apps \
        where tenant = 'default' and deleted_at is null \
        limit $1 offset $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *db)
    .await;

    match result {
        Ok(entities) => {
            debug!("Successfully retrieved apps: {:?}", entities);
            let mut apps = vec![];
            for entity in entities {
                apps.push(App {
                    name: entity.name,
                    id: entity.id.to_string(),
                    slug: entity.slug,
                    created_at: entity.created_at,
                    updated_at: entity.updated_at,
                })
            }
            Ok(apps)
        }
        Err(error) => {
            error!("Error retrieving apps. Error: {:?}", error);
            Err(map_sqlx_error(error))
        }
    }
}

pub async fn create_app(
    mut db: Connection<ConfigMonkeyDb>,
    slug: &str,
    name: &str,
) -> Result<App, AppsRepoError> {
    let result = sqlx::query_as::<_, AppEntity>(
        "insert into apps(tenant, slug, name) values ($1, $2, $3) returning id, slug, name, created_at, updated_at",
    )
    .bind("default")
    .bind(slug)
    .bind(name)
    .fetch_one(&mut *db)
    .await;

    match result {
        Ok(entity) => {
            debug!("Successfully created app: {:?}", entity);
            Ok(App {
                name: entity.name,
                id: entity.id.to_string(),
                slug: entity.slug,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
        }
        Err(error) => {
            error!("Error creating app with slug: {}. Error: {:?}", slug, error);
            Err(map_sqlx_error(error))
        }
    }
}

pub async fn delete_app(
    mut db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<(), AppsRepoError> {
    let result = sqlx::query("delete from apps where tenant = $1 and slug = $2")
        .bind("default")
        .bind(slug)
        .execute(&mut *db)
        .await;

    match result {
        Ok(_result) => {
            debug!("Successfully deleted app with slug: {}", slug);
            Ok(())
        }
        Err(error) => {
            error!("Error deleting app with slug: {}. Error: {:?}", slug, error);
            Err(map_sqlx_error(error))
        }
    }
}
