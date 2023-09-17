use std::borrow::Cow;

use crate::models::config::{Config, ConfigValue, ConfigVersion};
use chrono::{DateTime, Utc};
use rocket::error;
use rocket_db_pools::sqlx::{self};
use sqlx::{pool::PoolConnection, types::Uuid, Error, Postgres};

#[derive(Debug)]
pub enum ConfigsRepoError {
    AlreadyExists,
    NotFound,
    Unknown,
}

#[derive(sqlx::FromRow, Debug)]
struct ConfigEntity {
    pub id: Uuid,
    pub key: String,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
struct ValueEntity {
    pub id: Uuid,
    pub value: String,
    pub r#type: ValueTypeEntity,
    pub version: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "value_type", rename_all = "lowercase")]
enum ValueTypeEntity {
    String,
    Boolean,
    Number,
}

fn map_sqlx_error(error: Error) -> ConfigsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            Some(Cow::Borrowed("23505")) => ConfigsRepoError::AlreadyExists,
            _ => ConfigsRepoError::Unknown,
        },
        Error::RowNotFound => ConfigsRepoError::NotFound,
        _ => ConfigsRepoError::Unknown,
    }
}

fn to_value_type_entity(config_value: ConfigValue) -> ValueTypeEntity {
    match config_value {
        ConfigValue::Boolean(_) => ValueTypeEntity::Boolean,
        ConfigValue::String(_) => ValueTypeEntity::String,
        ConfigValue::Number(_) => ValueTypeEntity::Number,
    }
}

fn to_config_value(value_type: ValueTypeEntity, value: String) -> ConfigValue {
    match value_type {
        ValueTypeEntity::String => ConfigValue::String(value),
        ValueTypeEntity::Boolean => ConfigValue::Boolean(value.parse::<bool>().unwrap()),
        ValueTypeEntity::Number => ConfigValue::Number(value.parse::<f64>().unwrap()),
    }
}

pub async fn create_config(
    db: &mut PoolConnection<Postgres>,
    domain_id: &str,
    key: &str,
) -> Result<Config, ConfigsRepoError> {
    let create_config_result = sqlx::query_as::<_, ConfigEntity>(
        "insert into configs(domain_id, key) \
                        values($1::uuid, $2) \
                        returning id, key, created_at",
    )
    .bind(domain_id)
    .bind(key)
    .fetch_one(&mut *db)
    .await;

    match create_config_result {
        Err(err) => {
            error!("[create_config] Error creating config: {:?}", err);
            Err(map_sqlx_error(err))
        }
        Ok(config) => Ok(Config {
            id: config.id.to_string(),
            key: config.key,
            created_at: config.created_at,
        }),
    }
}

pub async fn get_configs(
    db: &mut PoolConnection<Postgres>,
    domain_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Vec<Config>, ConfigsRepoError> {
    let get_configs_result = sqlx::query_as::<_, ConfigEntity>(
        "select id, key, created_at from configs where domain_id = $1::uuid limit $2 offset $3",
    )
    .bind(domain_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *db)
    .await;

    match get_configs_result {
        Ok(configs) => {
            let mut result = vec![];
            for config in configs {
                result.push(Config {
                    id: config.id.to_string(),
                    key: config.key,
                    created_at: config.created_at,
                })
            }
            Ok(result)
        }
        Err(err) => {
            error!("[get_configs] Error retrieving configs: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}

pub async fn get_config(
    db: &mut PoolConnection<Postgres>,
    domain_id: &str,
    key: &str,
) -> Result<Config, ConfigsRepoError> {
    let get_config_result = sqlx::query_as::<_, ConfigEntity>(
        "select id, key, created_at from configs where domain_id = $1::uuid and key = $2",
    )
    .bind(domain_id)
    .bind(key)
    .fetch_one(&mut *db)
    .await;

    match get_config_result {
        Ok(config) => Ok(Config {
            id: config.id.to_string(),
            key: config.key,
            created_at: config.created_at,
        }),
        Err(err) => {
            error!("[get_donfig] Error retrieving config: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}

pub async fn delete_config(
    db: &mut PoolConnection<Postgres>,
    config_id: &str,
) -> Result<(), ConfigsRepoError> {
    let result = sqlx::query("delete from configs where id = $1::uuid")
        .bind(config_id)
        .execute(&mut *db)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                error!("[delete_config] Config not found: {}", config_id);
                return Err(ConfigsRepoError::NotFound);
            }
            Ok(())
        }
        Err(err) => {
            error!("[delete_config] Error deleting config: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}

// Versions

pub async fn create_version(
    db: &mut PoolConnection<Postgres>,
    config_id: &str,
    config_value: ConfigValue,
) -> Result<ConfigVersion, ConfigsRepoError> {
    let create_version_result =  sqlx::query_as::<_, ValueEntity>(
              "with latest_version as (select version from values where config_id = $1::uuid order by version desc limit 1) \
              insert into values(config_id, value, type, version) \
              values($1::uuid, $2, $3::value_type, coalesce((select version from latest_version), 0) + 1) \
              returning id, value, type, version, created_at",
            )
            .bind(config_id)
            .bind(config_value.to_string())
            .bind(to_value_type_entity(config_value))
            .fetch_one(&mut *db)
            .await;

    match create_version_result {
        Err(err) => {
            error!("[create_version] Error inserting value: {:?}", err);
            return Err(map_sqlx_error(err));
        }
        Ok(value) => Ok(ConfigVersion {
            id: value.id.to_string(),
            version: value.version,
            value: to_config_value(value.r#type, value.value),
            created_at: value.created_at,
        }),
    }
}

pub async fn get_versions(
    db: &mut PoolConnection<Postgres>,
    config_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Vec<ConfigVersion>, ConfigsRepoError> {
    let get_versions_result = sqlx::query_as::<_, ValueEntity>(
        "select id, value, type, version, created_at from values where config_id = $1::uuid order by version desc limit $2 offset $3",
    )
    .bind(config_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&mut *db)
    .await;

    match get_versions_result {
        Ok(versions) => {
            let mut result = vec![];
            for version in versions {
                result.push(ConfigVersion {
                    id: version.id.to_string(),
                    version: version.version,
                    value: to_config_value(version.r#type, version.value),
                    created_at: version.created_at,
                })
            }
            Ok(result)
        }
        Err(err) => {
            error!("[get_versions] Error retrieving versions: {:?}", err);
            Err(map_sqlx_error(err))
        }
    }
}
