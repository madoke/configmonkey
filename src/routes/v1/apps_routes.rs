use crate::{
    db::db::ConfigMonkeyDb,
    services::apps_service::{self, AppsServiceError},
};
use chrono::{DateTime, Utc};
use rocket::{
    delete, get,
    http::Status,
    post,
    response::Responder,
    serde::{json::Json, Deserialize, Serialize},
};
use rocket_db_pools::Connection;

use super::{dtos::PaginationDto, errors::RoutesError};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAppInput<'a> {
    slug: &'a str,
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAppDto {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetAppsDto {
    pub data: Vec<GetAppDto>,
    pub pagination: PaginationDto,
}

fn to_http_status(error: &AppsServiceError) -> Status {
    match error {
        AppsServiceError::DuplicateSlug => Status::Conflict,
        AppsServiceError::InvalidName => Status::BadRequest,
        AppsServiceError::InvalidSlug => Status::BadRequest,
        _ => Status::InternalServerError,
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetAppsResponse(Json<GetAppsDto>);

#[get("/v1/apps?<limit>&<offset>")]
pub async fn get_apps(
    db: Connection<ConfigMonkeyDb>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<GetAppsResponse, RoutesError> {
    let result = apps_service::get_apps(db, limit, offset).await;
    let mut appdtos = vec![];

    return match result {
        Ok(list) => {
            for app in list.items {
                appdtos.push(GetAppDto {
                    name: app.name,
                    slug: app.slug,
                    id: app.id,
                    created_at: app.created_at,
                    updated_at: app.updated_at,
                });
            }
            Ok(GetAppsResponse(Json(GetAppsDto {
                data: appdtos,
                pagination: PaginationDto {
                    count: list.count,
                    offset: list.offset,
                    limit: list.limit,
                    next: if let Some(next_offset) = list.next_offset {
                        Some(format!(
                            "/v1/apps?limit={}&offset={}",
                            list.limit, next_offset
                        ))
                    } else {
                        None
                    },
                    prev: if let Some(prev_offset) = list.prev_offset {
                        Some(format!(
                            "/v1/apps?limit={}&offset={}",
                            list.limit, prev_offset
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
pub struct CreateAppSuccess(Json<GetAppDto>);

#[post("/v1/apps", format = "application/json", data = "<input>")]
pub async fn create_app(
    db: Connection<ConfigMonkeyDb>,
    input: Json<CreateAppInput<'_>>,
) -> Result<CreateAppSuccess, RoutesError> {
    let result = apps_service::create_app(db, input.slug, input.name).await;

    return match result {
        Ok(app) => Ok(CreateAppSuccess(Json(GetAppDto {
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
pub struct DeleteAppSuccess(());

#[delete("/v1/apps/<slug>")]
pub async fn delete_app(
    db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<DeleteAppSuccess, RoutesError> {
    let result = apps_service::delete_app(db, slug).await;

    return match result {
        Ok(()) => Ok(DeleteAppSuccess(())),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}
