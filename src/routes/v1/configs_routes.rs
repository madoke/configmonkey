use crate::db::db::ConfigMonkeyDb;
use crate::services::configs_service::{self, ConfigsServiceError};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use super::errors::RoutesError;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetConfigDto {
    pub id: String,
    pub config: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn to_http_status(error: &ConfigsServiceError) -> Status {
    match error {
        ConfigsServiceError::AppOrEnvNotFound => Status::NotFound,
        ConfigsServiceError::InvalidConfigFormat => Status::BadRequest,
        ConfigsServiceError::ConfigAlreadyExists => Status::Conflict,
        ConfigsServiceError::Unknown => Status::InternalServerError,
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetConfigResponse(Json<GetConfigDto>);

#[get("/v1/configs/<app_slug>/<env_slug>")]
pub async fn get_config(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
) -> Result<GetConfigResponse, RoutesError> {
    let result = configs_service::get_config(db, app_slug, env_slug).await;
    return match result {
        Ok(config) => Ok(GetConfigResponse(Json(GetConfigDto {
            id: config.id,
            config: config.config,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }))),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreateConfigSuccess(Json<GetConfigDto>);

#[post(
    "/v1/configs/<app_slug>/<env_slug>",
    format = "application/json",
    data = "<input>"
)]
pub async fn create_config(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
    input: &str,
) -> Result<CreateConfigSuccess, RoutesError> {
    let result = configs_service::create_config(db, app_slug, env_slug, input).await;

    return match result {
        Ok(config) => Ok(CreateConfigSuccess(Json(GetConfigDto {
            id: config.id,
            config: config.config,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }))),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}
