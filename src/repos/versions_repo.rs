use crate::models::config::{ConfigValue, ConfigVersion};
use chrono::{DateTime, Utc};
use rocket::error;
use rocket_db_pools::sqlx::{self};
use sqlx::{pool::PoolConnection, types::Uuid, Error, Postgres};

#[derive(Debug)]
pub enum VersionsRepoError {
    NotFound,
    Unknown,
}

#[derive(sqlx::FromRow, Debug)]
struct VersionEntity {
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
    Float,
    Integer,
}

fn map_sqlx_error(error: Error) -> VersionsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            _ => VersionsRepoError::Unknown,
        },
        Error::RowNotFound => VersionsRepoError::NotFound,
        _ => VersionsRepoError::Unknown,
    }
}

fn to_value_type_entity(config_value: ConfigValue) -> ValueTypeEntity {
    match config_value {
        ConfigValue::Boolean(_) => ValueTypeEntity::Boolean,
        ConfigValue::String(_) => ValueTypeEntity::String,
        ConfigValue::Float(_) => ValueTypeEntity::Float,
        ConfigValue::Integer(_) => ValueTypeEntity::Integer,
    }
}

fn to_config_value(value_type: ValueTypeEntity, value: String) -> ConfigValue {
    match value_type {
        ValueTypeEntity::String => ConfigValue::String(value),
        ValueTypeEntity::Boolean => ConfigValue::Boolean(value.parse::<bool>().unwrap()),
        ValueTypeEntity::Float => ConfigValue::Float(value.parse::<f64>().unwrap()),
        ValueTypeEntity::Integer => ConfigValue::Integer(value.parse::<i64>().unwrap()),
    }
}

pub async fn create_version(
    db: &mut PoolConnection<Postgres>,
    config_id: &str,
    config_value: ConfigValue,
) -> Result<ConfigVersion, VersionsRepoError> {
    let create_version_result =  sqlx::query_as::<_, VersionEntity>(
              "with latest_version as (select version from versions where config_id = $1::uuid order by version desc limit 1) \
              insert into versions(config_id, value, type, version) \
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
) -> Result<Vec<ConfigVersion>, VersionsRepoError> {
    let get_versions_result = sqlx::query_as::<_, VersionEntity>(
        "select id, value, type, version, created_at from versions where config_id = $1::uuid order by version desc limit $2 offset $3",
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
