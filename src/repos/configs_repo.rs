use std::borrow::Cow;

use crate::{db::db::ConfigMonkeyDb, models::config::Config};
use chrono::{DateTime, Utc};
use rocket::{
    error,
    log::private::debug,
    serde::json::{
        serde_json::{self, Map, Value},
        to_string,
    },
};
use rocket_db_pools::{
    sqlx::{self, types::Uuid},
    Connection,
};
use sqlx::{types::Json, Error};

pub enum ConfigsRepoError {
    Unknown,
    AppOrEnvNotFound,
    ConfigAlreadyExists,
    InvalidConfigJson,
}

fn map_sqlx_error(error: Error) -> ConfigsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            Some(Cow::Borrowed("23505")) => ConfigsRepoError::ConfigAlreadyExists,
            _ => ConfigsRepoError::Unknown,
        },
        Error::RowNotFound => ConfigsRepoError::AppOrEnvNotFound,
        Error::ColumnDecode {
            index: _,
            source: _,
        } => ConfigsRepoError::InvalidConfigJson,
        _ => ConfigsRepoError::Unknown,
    }
}

#[derive(sqlx::FromRow, Debug)]
struct ConfigEntity {
    pub id: Uuid,
    pub config: Json<Map<String, Value>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn get_config(
    mut db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
) -> Result<Config, ConfigsRepoError> {
    let result = sqlx::query_as::<_, ConfigEntity>(
        "select c.id, c.config, c.created_at, c.updated_at from configs c \
        join envs e on e.id = c.env_id \
        join apps a on a.id = e.app_id \
        where a.slug = $1 and e.slug = $2",
    )
    .bind(app_slug)
    .bind(env_slug)
    .fetch_one(&mut *db)
    .await;

    match result {
        Ok(config) => {
            debug!("Successfully retrieved config: {:?}", config);
            Ok(Config {
                id: config.id.to_string(),
                config: to_string(&config.config).unwrap(),
                created_at: config.created_at,
                updated_at: config.updated_at,
            })
        }
        Err(err) => {
            error!("Error retrieving config. Error: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}

pub async fn create_config(
    mut db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
    config: serde_json::Value,
) -> Result<Config, ConfigsRepoError> {
    print!("{}", config);
    let result = sqlx::query_as::<_, ConfigEntity>(
        "with get_env_id as (select e.id from envs e join apps a on a.id = e.app_id where a.slug = $1 and e.slug = $2) \
        insert into configs(env_id, config) \
        (select id, $3 from get_env_id) \
        returning id, config, created_at, updated_at",
    )
    .bind(app_slug)
    .bind(env_slug)
    .bind(config)
    .fetch_one(&mut *db)
    .await;

    match result {
        Ok(entity) => {
            debug!("Successfully created config: {:?}", entity);
            Ok(Config {
                id: entity.id.to_string(),
                config: to_string(&entity.config).unwrap(),
                created_at: entity.created_at,
                updated_at: entity.updated_at,
            })
        }
        Err(err) => {
            error!("Error creating config. Error: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}

pub async fn delete_config(
    mut db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
) -> Result<(), ConfigsRepoError> {
    let result = sqlx::query(
        "delete from configs c \
        using apps a, envs e \
        where a.id = e.app_id and e.id = c.env_id and a.slug = $1 and e.slug = $2",
    )
    .bind(app_slug)
    .bind(env_slug)
    .execute(&mut *db)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!(
                    "Env {} or app {} not found or no config exists",
                    env_slug, app_slug
                );
                return Err(ConfigsRepoError::AppOrEnvNotFound);
            }
            debug!(
                "Successfully deleted config for app {} and env {} ",
                app_slug, env_slug
            );
            Ok(())
        }
        Err(err) => {
            error!(
                "Error deleting config for app {} and env {}: {:?}",
                app_slug, env_slug, err
            );
            Err(map_sqlx_error(err))
        }
    }
}
