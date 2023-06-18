use std::io::Cursor;

use super::shared_dtos::ErrorMessageDto;
use crate::db::db::ConfigMonkeyDb;
use crate::services::configs_svc::{self, ConfigsServiceError};
use chrono::{DateTime, Utc};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::serde_json::to_string;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, Response};
use rocket_db_pools::Connection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetConfigDto {
    pub id: String,
    pub config: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct ConfigsRoutesError(ConfigsServiceError);

impl<'a> Responder<'a, 'static> for ConfigsRoutesError {
    fn respond_to(self, _: &'a Request<'_>) -> rocket::response::Result<'static> {
        let ConfigsRoutesError(apps_service_error) = self;
        let status = match apps_service_error {
            _ => Status::InternalServerError,
        };
        let response_body = to_string(&ErrorMessageDto {
            code: apps_service_error.code(),
            message: apps_service_error.message(),
        })
        .unwrap();
        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(response_body.len(), Cursor::new(response_body))
            .ok()
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
) -> Result<GetConfigResponse, ConfigsRoutesError> {
    let result = configs_svc::get_config(db, app_slug, env_slug).await;
    return match result {
        Ok(config) => Ok(GetConfigResponse(Json(GetConfigDto {
            id: config.id,
            config: config.config,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }))),
        Err(err) => Err(ConfigsRoutesError(err)),
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
) -> Result<CreateConfigSuccess, ConfigsRoutesError> {
    let result = configs_svc::create_config(db, app_slug, env_slug, input).await;

    return match result {
        Ok(config) => Ok(CreateConfigSuccess(Json(GetConfigDto {
            id: config.id,
            config: config.config,
            created_at: config.created_at,
            updated_at: config.updated_at,
        }))),
        Err(err) => Err(ConfigsRoutesError(err)),
    };
}
