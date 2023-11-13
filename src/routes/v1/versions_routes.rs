use crate::db::db::ConfigMonkeyDb;
use crate::models::config::ConfigValue;
use crate::services::versions_service::{self, VersionsServiceError};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::json;
use rocket::serde::{
    json::{Json, Value},
    Deserialize, Serialize,
};

use rocket::{get, post};
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
pub struct CreateVersionDto {
    pub value: Value,
}

fn to_http_status(error: &VersionsServiceError) -> Status {
    match error {
        VersionsServiceError::ConfigNotFound => Status::NotFound,
        VersionsServiceError::DomainNotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

fn to_value(config_value: ConfigValue) -> Value {
    match config_value {
        ConfigValue::String(v) => json!(v),
        ConfigValue::Boolean(v) => json!(v),
        ConfigValue::Integer(v) => json!(v),
        ConfigValue::Float(v) => json!(v),
    }
}

fn from_value(value: &Value) -> ConfigValue {
    match value {
        Value::Number(v) => {
            if v.is_f64() {
                ConfigValue::Float(v.as_f64().unwrap())
            } else if v.is_i64() {
                ConfigValue::Integer(v.as_i64().unwrap() as i64)
            } else {
                ConfigValue::Integer(v.as_u64().unwrap() as i64)
            }
        }
        Value::String(v) => ConfigValue::String(v.to_string()),
        Value::Bool(v) => ConfigValue::Boolean(*v),
        _ => {
            panic!("Value type not supported!");
        }
    }
}

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

    let result = versions_service::create_version(db, domain_slug, key, config_value).await;

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
    let result = versions_service::get_versions(db, domain_slug, key, limit, offset).await;
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
                            "/v1/configs/{}/{}/versions?limit={}&offset={}",
                            domain_slug, key, versions.limit, next_offset
                        ))
                    } else {
                        None
                    },
                    prev: if let Some(prev_offset) = versions.prev_offset {
                        Some(format!(
                            "/v1/configs/{}/{}/versions?limit={}&offset={}",
                            domain_slug, key, versions.limit, prev_offset
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
