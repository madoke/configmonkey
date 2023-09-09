use crate::{
    db::db::ConfigMonkeyDb,
    models::config::{Config, ConfigType, ValueType},
};
use chrono::{DateTime, Utc};
use rocket::error;
use rocket_db_pools::{
    sqlx::{self},
    Connection,
};
use sqlx::{Error, types::Uuid};
use std::str::FromStr;

pub enum ConfigsRepoError {
    Unknown,
}

fn map_sqlx_error(error: Error) -> ConfigsRepoError {
    match error {
        Error::Database(err) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            // Some(Cow::Borrowed("23505")) => ConfigsRepoError::ConfigAlreadyExists,
            _ => ConfigsRepoError::Unknown,
        },
        // Error::RowNotFound => ConfigsRepoError::AppOrEnvNotFound,
        _ => ConfigsRepoError::Unknown,
    }
}

#[derive(sqlx::FromRow, Debug)]
struct ConfigEntity {
    pub id: Uuid,
    pub domain_id: Uuid,
    pub key: String,
    pub r#type: ConfigType,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
#[derive(sqlx::FromRow, Debug)]
struct ValueEntity {
    pub config_id: Uuid,
    pub value: String,
    pub r#type: ValueType,
    pub version: i32,
    pub created_at: DateTime<Utc>,
}

pub async fn create_config(
    mut db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    config_type: ConfigType,
    value_type: ValueType,
    value: &str,
) -> Result<Config, ConfigsRepoError> {
    let config_result = sqlx::query_as::<_, ConfigEntity>(
        "with domain_id as ( select id from domains where slug = $1 ) \
      select id, domain_id, key, type, parent_id, created_at from configs \
      where domain_id = (select id from domain_id) and key = $2",
    )
    .bind(domain_slug)
    .bind(key)
    .fetch_optional(&mut *db)
    .await;

    match config_result {
        Err(err) => {
            error!("Error verifying if config exists: {:?}", err);
            Err(map_sqlx_error(err))
        }
        Ok(optional_config) => {
            let config = match optional_config {
                Some(config) => config,
                None => {
                    let create_config_result = sqlx::query_as::<_, ConfigEntity>(
                        "with domain_id as ( select id from domains where slug = $1 ) \
                        insert into configs(domain_id, key, type, parent_id) \
                        values((select id from domain_id), $2, $3, null) \
                        returning id, domain_id, key, type, parent_id, created_at",
                    )
                    .bind(domain_slug)
                    .bind(key)
                    .bind(config_type)
                    .fetch_one(&mut *db)
                    .await;

                    match create_config_result {
                        Err(err) => {
                            error!("Error creating config: {:?}", err);
                            return Err(map_sqlx_error(err));
                        }
                        Ok(config) => config,
                    }
                }
            };

            let insert_value_result =  sqlx::query_as::<_, ValueEntity>(
              "with latest_version as (select version from values where config_id = $1 order by version desc limit 1) \
              insert into values(config_id, value, type, version) \
              values($1, $2, $3, coalesce((select version from latest_version), 0) + 1) \
              returning config_id, value, type, version, created_at",
            )
            .bind(config.id)
            .bind(value)
            .bind(value_type)
            .fetch_one(&mut *db)
            .await;

            match insert_value_result {
                Err(err) => {
                    error!("Error inserting value: {:?}", err);
                    return Err(map_sqlx_error(err));
                }
                Ok(value) => Ok(Config {
                    id: value.config_id.to_string(),
                    key: config.key,
                    config_type: config.r#type,
                    value_type: value.r#type,
                    version: value.version,
                    value: value.value,
                    created_at: config.created_at,
                    updated_at: value.created_at,
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
