use crate::db::db::ConfigMonkeyDb;
use crate::services::envs_service::{self, EnvsServiceError};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, post};
use rocket_db_pools::Connection;

use super::dtos::PaginationDto;
use super::errors::RoutesError;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetEnvDto {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetEnvsDto {
    pub data: Vec<GetEnvDto>,
    pub pagination: PaginationDto,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateEnvInput<'a> {
    slug: &'a str,
    name: &'a str,
}

fn to_http_status(error: &EnvsServiceError) -> Status {
    match error {
        EnvsServiceError::DuplicateSlug => Status::Conflict,
        EnvsServiceError::InvalidName => Status::BadRequest,
        EnvsServiceError::InvalidSlug => Status::BadRequest,
        EnvsServiceError::AppOrEnvNotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetEnvsResponse(Json<GetEnvsDto>);

#[get("/v1/envs/<app_slug>?<limit>&<offset>")]
pub async fn get_envs(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<GetEnvsResponse, RoutesError> {
    let result = envs_service::get_envs(db, app_slug, limit, offset).await;
    let mut appdtos = vec![];

    return match result {
        Ok(list) => {
            for app in list.items {
                appdtos.push(GetEnvDto {
                    name: app.name,
                    slug: app.slug,
                    id: app.id,
                    created_at: app.created_at,
                    updated_at: app.updated_at,
                });
            }
            Ok(GetEnvsResponse(Json(GetEnvsDto {
                data: appdtos,
                pagination: PaginationDto {
                    count: list.count,
                    offset: list.offset,
                    limit: list.limit,
                    next: if let Some(next_offset) = list.next_offset {
                        Some(format!(
                            "/v1/envs/{}?limit={}&offset={}",
                            app_slug, list.limit, next_offset
                        ))
                    } else {
                        None
                    },
                    prev: if let Some(prev_offset) = list.prev_offset {
                        Some(format!(
                            "/v1/envs/{}?limit={}&offset={}",
                            app_slug, list.limit, prev_offset
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
#[response(status = 201, content_type = "json")]
pub struct CreateEnvSuccess(Json<GetEnvDto>);

#[post("/v1/envs/<app_slug>", format = "application/json", data = "<input>")]
pub async fn create_env(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    input: Json<CreateEnvInput<'_>>,
) -> Result<CreateEnvSuccess, RoutesError> {
    let result = envs_service::create_env(db, app_slug, input.slug, input.name).await;

    return match result {
        Ok(app) => Ok(CreateEnvSuccess(Json(GetEnvDto {
            name: app.name,
            slug: app.slug,
            id: app.id,
            created_at: app.created_at,
            updated_at: app.updated_at,
        }))),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}

#[derive(Responder)]
#[response(status = 204, content_type = "json")]
pub struct DeleteEnvSuccess(());

#[delete("/v1/envs/<app_slug>/<env_slug>")]
pub async fn delete_env(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    env_slug: &str,
) -> Result<DeleteEnvSuccess, RoutesError> {
    let result = envs_service::delete_env(db, app_slug, env_slug).await;

    return match result {
        Ok(()) => Ok(DeleteEnvSuccess(())),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}
