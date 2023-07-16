use crate::{db::db::ConfigMonkeyDb, models::env::Env};
use chrono::{DateTime, Utc};
use rocket::{error, log::private::debug};
use rocket_db_pools::{
    sqlx::{self, types::Uuid},
    Connection,
};
use sqlx::Error;
use std::borrow::Cow;

pub enum EnvsRepoError {
    Unknown,
    DuplicateSlug,
    AppOrEnvNotFound,
    EntityHasChildren,
}

fn map_sqlx_error(error: Error) -> EnvsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            Some(Cow::Borrowed("23505")) => EnvsRepoError::DuplicateSlug,
            Some(Cow::Borrowed("23503")) => EnvsRepoError::EntityHasChildren,
            _ => EnvsRepoError::Unknown,
        },
        Error::RowNotFound => EnvsRepoError::AppOrEnvNotFound,
        _ => EnvsRepoError::Unknown,
    }
}

#[derive(sqlx::FromRow, Debug)]
struct EnvEntity {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_envs(
    mut db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    limit: i32,
    offset: i32,
) -> Result<Vec<Env>, EnvsRepoError> {
    let result = sqlx::query_as::<_, EnvEntity>(
        "select e.id, e.slug, e.name, e.created_at, e.updated_at from envs e \
        join apps a on a.tenant = 'default' and a.id = e.app_id \
        where a.slug = $1 \
        limit $2 offset $3",
    )
    .bind(app_slug)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *db)
    .await;

    match result {
        Ok(entities) => {
            debug!("Successfully retrieved envs: {:?}", entities);
            let mut envs = vec![];
            for entity in entities {
                envs.push(Env {
                    name: entity.name,
                    id: entity.id.to_string(),
                    slug: entity.slug,
                    created_at: entity.created_at,
                    updated_at: entity.updated_at,
                })
            }
            Ok(envs)
        }
        Err(err) => {
            error!("Error retrieving envs. Error: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}

pub async fn create_env(
    mut db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    slug: &str,
    name: &str,
) -> Result<Env, EnvsRepoError> {
    let result = sqlx::query_as::<_, EnvEntity>(
        "with get_app_id as (select id from apps where slug = $1) \
        insert into envs(app_id, slug, name) \
        (select id, $2, $3 from get_app_id) \
        returning id, slug, name, created_at, updated_at",
    )
    .bind(app_slug)
    .bind(slug)
    .bind(name)
    .fetch_one(&mut *db)
    .await;

    match result {
        Ok(entity) => {
            debug!("Successfully created env: {:?}", entity);
            Ok(Env {
                name: entity.name,
                id: entity.id.to_string(),
                slug: entity.slug,
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
        }
        Err(err) => {
            error!("Error creating env with slug: {}. Error: {:?}", slug, err);
            Err(map_sqlx_error(err))
        }
    }
}

pub async fn delete_env(
    mut db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    slug: &str,
) -> Result<(), EnvsRepoError> {
    let result = sqlx::query(
        "delete from envs e  
        using apps a \
        where a.id = e.app_id and a.slug = $1 and e.slug = $2",
    )
    .bind(app_slug)
    .bind(slug)
    .execute(&mut *db)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("Env {} or app {} not found", slug, app_slug);
                return Err(EnvsRepoError::AppOrEnvNotFound);
            }
            debug!("Successfully deleted env with slug: {}", slug);
            Ok(())
        }
        Err(err) => {
            error!(
                "Error deleting env {} on app {}. Error: {:?}",
                slug, app_slug, err
            );
            Err(map_sqlx_error(err))
        }
    }
}
