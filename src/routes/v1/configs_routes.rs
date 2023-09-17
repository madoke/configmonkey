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

use rocket::{delete, get, post};
use rocket_db_pools::Connection;

use super::dtos::{PaginatedListDto, PaginationDto};
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
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateConfigDto {
    pub key: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateVersionDto {
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

    let result = configs_service::create_config(db, domain_slug, key).await;

    match result {
        Ok(config) => Ok(CreateConfigSuccess(Json(GetConfigDto {
            key: config.key,
            created_at: config.created_at,
        }))),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetConfigResponse(Json<GetConfigDto>);

#[get("/v1/configs/<domain_slug>/<key>")]
pub async fn get_config(
    db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
) -> Result<GetConfigResponse, RoutesError> {
    let result = configs_service::get_config(db, domain_slug, key).await;
    return match result {
        Ok(config) => Ok(GetConfigResponse(Json(GetConfigDto {
            key: config.key,
            created_at: config.created_at,
        }))),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetConfigsResponse(Json<PaginatedListDto<GetConfigDto>>);

#[get("/v1/configs/<domain_slug>?<limit>&<offset>")]
pub async fn get_configs(
    db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<GetConfigsResponse, RoutesError> {
    let result = configs_service::get_configs(db, domain_slug, limit, offset).await;
    return match result {
        Ok(configs) => {
            let mut result = vec![];
            for config in configs.items {
                result.push(GetConfigDto {
                    key: config.key,
                    created_at: config.created_at,
                })
            }
            Ok(GetConfigsResponse(Json(PaginatedListDto {
                data: result,
                pagination: PaginationDto {
                    count: configs.count,
                    offset: configs.offset,
                    limit: configs.limit,
                    next: if let Some(next_offset) = configs.next_offset {
                        Some(format!(
                            "/v1/domains?limit={}&offset={}",
                            configs.limit, next_offset
                        ))
                    } else {
                        None
                    },
                    prev: if let Some(prev_offset) = configs.prev_offset {
                        Some(format!(
                            "/v1/domains?limit={}&offset={}",
                            configs.limit, prev_offset
                        ))
                    } else {
                        None
                    },
                },
            })))
        }
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}

#[derive(Responder)]
#[response(status = 204, content_type = "json")]
pub struct DeleteConfigSuccess(());

#[delete("/v1/configs/<domain_slug>/<key>")]
pub async fn delete_config(
    db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
) -> Result<DeleteConfigSuccess, RoutesError> {
    let result = configs_service::delete_config(db, domain_slug, key).await;

    return match result {
        Ok(()) => Ok(DeleteConfigSuccess(())),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}

// Versions

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreateVersionSuccess(Json<GetVersionDto>);

#[post(
    "/v1/configs/<domain_slug>/<key>/versions",
    format = "application/json",
    data = "<input>"
)]
pub async fn create_version(
    db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    input: Json<CreateVersionDto>,
) -> Result<CreateVersionSuccess, RoutesError> {
    let config_value = from_value(&input.value);

    let result = configs_service::create_version(db, domain_slug, key, config_value).await;

    match result {
        Ok(version) => Ok(CreateVersionSuccess(Json(GetVersionDto {
            id: version.version,
            created_at: version.created_at,
            value: to_value(version.value),
        }))),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetVersionsResponse(Json<PaginatedListDto<GetVersionDto>>);

#[get("/v1/configs/<domain_slug>/<key>/versions?<limit>&<offset>")]
pub async fn get_versions(
    db: Connection<ConfigMonkeyDb>,
    domain_slug: &str,
    key: &str,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<GetVersionsResponse, RoutesError> {
    let result = configs_service::get_versions(db, domain_slug, key, limit, offset).await;
    return match result {
        Ok(versions) => {
            let mut result = vec![];
            for version in versions.items {
                result.push(GetVersionDto {
                    id: version.version,
                    value: to_value(version.value),
                    created_at: version.created_at,
                })
            }
            Ok(GetVersionsResponse(Json(PaginatedListDto {
                data: result,
                pagination: PaginationDto {
                    count: versions.count,
                    offset: versions.offset,
                    limit: versions.limit,
                    next: if let Some(next_offset) = versions.next_offset {
                        Some(format!(
                            "/v1/domains?limit={}&offset={}",
                            versions.limit, next_offset
                        ))
                    } else {
                        None
                    },
                    prev: if let Some(prev_offset) = versions.prev_offset {
                        Some(format!(
                            "/v1/domains?limit={}&offset={}",
                            versions.limit, prev_offset
                        ))
                    } else {
                        None
                    },
                },
            })))
        }
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}

