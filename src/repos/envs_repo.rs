use std::borrow::Cow;

use crate::{
    db::db::ConfigMonkeyDb,
    models::{app, env::Env},
};
use chrono::{DateTime, Utc};
use rocket::log::private::debug;
use rocket_db_pools::{
    sqlx::{self, types::Uuid},
    Connection,
};
use sqlx::Error;

pub enum EnvsRepoError {
    Unknown,
    DuplicateSlug,
}

fn map_sqlx_error(error: Error) -> EnvsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            Some(Cow::Borrowed("23505")) => EnvsRepoError::DuplicateSlug,
            _ => EnvsRepoError::Unknown,
        },
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
        where a.slug = $1 and a.deleted_at is null and e.deleted_at is null \
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
        Err(error) => {
            error!("Error retrieving envs. Error: {:?}", error);
            Err(map_sqlx_error(error))
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
        Err(error) => {
            error!("Error creating env with slug: {}. Error: {:?}", slug, error);
            Err(map_sqlx_error(error))
        }
    }
}
