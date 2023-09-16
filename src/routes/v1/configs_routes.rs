use crate::db::db::ConfigMonkeyDb;
use crate::models::config::ConfigValue;
use crate::services::configs_service::{self, ConfigsServiceError};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::json;
use rocket::serde::{
    json::{Json, Value},
    Deserialize, Serialize,
};

use rocket::post;
use rocket_db_pools::Connection;

use super::errors::RoutesError;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetVersionDto {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub value: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetConfigDto {
    pub key: String,
    pub created_at: DateTime<Utc>,
    pub versions: Vec<GetVersionDto>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateConfigDto {
    pub key: String,
    pub value: Value,
}

fn to_http_status(error: &ConfigsServiceError) -> Status {
    match error {
        _ => Status::InternalServerError,
    }
}

fn to_value(config_value: ConfigValue) -> Value {
    match config_value {
        ConfigValue::String(s) => json!(s),
        ConfigValue::Boolean(b) => json!(b),
        ConfigValue::Number(n) => json!(n),
    }
}

fn from_value(value: &Value) -> ConfigValue {
    match value {
        Value::String(v) => ConfigValue::String(v.to_string()),
        Value::Bool(v) => ConfigValue::Boolean(*v),
        _ => {
            panic!("Value type not supported!");
        }
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreateConfigSuccess(Json<GetConfigDto>);

#[post(
    "/v1/configs/<domain_slug>",
    format = "application/json",
    data = "<input>"
)]
pub async fn create_config(
    db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    input: Json<CreateConfigDto>,
) -> Result<CreateConfigSuccess, RoutesError> {
    let key = input.key.as_str();
    let config_value = from_value(&input.value);

    let result = configs_service::create_config(db, domain_slug, key, config_value).await;

    match result {
        Ok(config) => {
            let mut versions: Vec<GetVersionDto> = Vec::new();
            for version in config.versions {
                versions.push(GetVersionDto {
                    id: version.index,
                    created_at: version.created_at,
                    value: to_value(version.value),
                })
            }
            Ok(CreateConfigSuccess(Json(GetConfigDto {
                key: config.key,
                created_at: config.created_at,
                versions: versions,
            })))
        }
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    }
}

// #[derive(Responder)]
// #[response(status = 200, content_type = "json")]
// pub struct GetConfigResponse(Json<GetConfigsDto>);

// #[get("/v1/configs/<domain_slug>")]
// pub async fn get_config(
//     db: Connection<ConfigMonkeyDb>,
//     domain_slug: &str,
// ) -> Result<GetConfigResponse, RoutesError> {
//     let result = configs_service::get_config(db, app_slug, env_slug).await;
//     return match result {
//         Ok(config) => Ok(GetConfigResponse(Json(GetConfigDto {
//             key: config.key,
//             version: config.version,
//             config_type: config.config_type.to_string(),
//             created_at: config.created_at,
//             updated_at: config.updated_at,
//         }))),
//         Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
//     };
// }

// #[derive(Responder)]
// #[response(status = 204, content_type = "json")]
// pub struct DeleteConfigSuccess(());

// #[delete("/v1/configs/<app_slug>/<env_slug>")]
// pub async fn delete_config(
//     db: Connection<ConfigMonkeyDb>,
//     app_slug: &str,
//     env_slug: &str,
// ) -> Result<DeleteConfigSuccess, RoutesError> {
//     let result = configs_service::delete_config(db, app_slug, env_slug).await;

//     return match result {
//         Ok(()) => Ok(DeleteConfigSuccess(())),
//         Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
//     };
// }
