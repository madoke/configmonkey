use crate::{db::db::ConfigMonkeyDb, models::config::Config};
use chrono::{DateTime, Utc};
use rocket::{
    log::private::debug,
    serde::json::{
        serde_json::{Map, Value},
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
}

fn map_sqlx_error(error: Error) -> ConfigsRepoError {
    match error {
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
        Err(error) => {
            error!("Error retrieving config. Error: {:?}", error);
            Err(map_sqlx_error(error))
        }
    }
}

pub async fn create_config(
    mut db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
    config: &str,
) -> Result<Config, ConfigsRepoError> {
    let result = sqlx::query_as::<_, ConfigEntity>(
        "with get_env_id as (select e.id from envs e join apps a on a.id = e.app_id where a.slug = $1 and e.slug = $2) \
        insert into configs(env_id, config) \
        (select id, $3 from get_env_id) \
        returning id, config, created_at, updated_at",
    )
    .bind(app_slug)
    .bind(env_slug)
    .bind(Json(config))
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
        Err(error) => {
            error!("Error creating config. Error: {:?}", error);
            Err(map_sqlx_error(error))
        }
    }
}
