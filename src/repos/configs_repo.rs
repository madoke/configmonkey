use crate::models::config::{Config, ConfigValue, ConfigVersion};
use chrono::{DateTime, Utc};
use rocket::error;
use rocket_db_pools::sqlx::{self};
use sqlx::{pool::PoolConnection, types::Uuid, Error, Postgres};

pub enum ConfigsRepoError {
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
    String, Boolean, Number
}

fn map_sqlx_error(error: Error) -> ConfigsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            // Some(Cow::Borrowed("23505")) => ConfigsRepoError::ConfigAlreadyExists,
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
    config_value: ConfigValue,
) -> Result<Config, ConfigsRepoError> {
    let config_result = sqlx::query_as::<_, ConfigEntity>(
        "select id, key, created_at from configs \
        where domain_id = $1::uuid and key = $2",
    )
    .bind(domain_id)
    .bind(key)
    .fetch_optional(&mut *db)
    .await;

    match config_result {
        Err(err) => {
            error!(
                "[create_config] Error verifying if config exists: {:?}",
                err
            );
            Err(map_sqlx_error(err))
        }
        Ok(optional_config) => {
            let config = match optional_config {
                Some(config) => config,
                None => {
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
                            return Err(map_sqlx_error(err));
                        }
                        Ok(config) => config,
                    }
                }
            };

            let insert_value_result =  sqlx::query_as::<_, ValueEntity>(
              "with latest_version as (select version from values where config_id = $1 order by version desc limit 1) \
              insert into values(config_id, value, type, version) \
              values($1, $2, $3::value_type, coalesce((select version from latest_version), 0) + 1) \
              returning id, value, type, version, created_at",
            )
            .bind(config.id)
            .bind(config_value.to_string())
            .bind(to_value_type_entity(config_value))
            .fetch_one(&mut *db)
            .await;

            match insert_value_result {
                Err(err) => {
                    error!("[create_config] Error inserting value: {:?}", err);
                    return Err(map_sqlx_error(err));
                }
                Ok(value) => Ok(Config {
                    id: config.id.to_string(),
                    key: config.key,
                    created_at: config.created_at,
                    versions: vec![ConfigVersion {
                        id: value.id.to_string(),
                        index: value.version,
                        value: to_config_value(value.r#type, value.value),
                        created_at: value.created_at,
                    }],
                }),
            }
        }
    }
}

// pub async fn get_configs(
//     mut db: Connection<ConfigMonkeyDb>,
//     app_slug: &str,
//     env_slug: &str,
//     limit: i32,
//     offset: i32,
// ) -> Result<Vec<Config>, ConfigsRepoError> {
//     let result = sqlx::query_as::<_, ConfigEntity>(
//         "with ecfgs as ( \
//                 select * from configs c \
//                 join envs e on e.id = c.env_id \
//                 join apps a on a.id = e.app_id \
//                 where a.slug = $1 and e.slug = $2 \
//             ), maxversion as ( \
//                 select key, max(version) as version from ecfgs \
//                 group by key \
//             ), v0created as ( \
//                 select key, created_at from ecfgs \
//                 where version = 0 \
//             ) \
//             select ecfgs.id, ecfgs.key, ecfgs.type, ecfgs.value, ecfgs.version, ecfgs.created_at as updated_at, v0created.created_at \
//             from ecfgs \
//             join maxversion on ecfgs.key = maxversion.key \
//             join v0created on ecfgs.key = v0created.key \
//             where ecfgs.version = maxversion.version \
//             limit $3 offset $4",
//     )
//     .bind(app_slug)
//     .bind(env_slug)
//     .bind(limit)
//     .bind(offset)
//     .fetch_all(&mut *db)
//     .await;

//     match result {
//         Ok(entities) => {
//             debug!("Successfully retrieved configs: {:?}", entities);
//             let mut configs = vec![];
//             for entity in entities {
//                 configs.push(Config {
//                     id: entity.id,
//                     key: entity.key,
//                     version: entity.version,
//                     value: entity.value,
//                     r#type: ConfigType::from_str(entity.r#type.as_str()).unwrap(),
//                     created_at: entity.created_at,
//                     updated_at: entity.updated_at,
//                 })
//             }
//             Ok(configs)
//         }
//         Err(err) => {
//             error!("Error retrieving configs. Error: {:?}", err);
//             Err(map_sqlx_error(err))
//         }
//     }
// }

// pub async fn delete_config(
//     mut db: Connection<ConfigMonkeyDb>,
//     app_slug: &str,
//     env_slug: &str,
// ) -> Result<(), ConfigsRepoError> {
//     let result = sqlx::query(
//         "delete from configs c \
//         using apps a, envs e \
//         where a.id = e.app_id and e.id = c.env_id and a.slug = $1 and e.slug = $2",
//     )
//     .bind(app_slug)
//     .bind(env_slug)
//     .execute(&mut *db)
//     .await;

//     match result {
//         Ok(result) => {
//             if result.rows_affected() == 0 {
//                 error!(
//                     "Env {} or app {} not found or no config exists",
//                     env_slug, app_slug
//                 );
//                 return Err(ConfigsRepoError::AppOrEnvNotFound);
//             }
//             debug!(
//                 "Successfully deleted config for app {} and env {} ",
//                 app_slug, env_slug
//             );
//             Ok(())
//         }
//         Err(err) => {
//             error!(
//                 "Error deleting config for app {} and env {}: {:?}",
//                 app_slug, env_slug, err
//             );
//             Err(map_sqlx_error(err))
//         }
//     }
// }
