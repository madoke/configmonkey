use std::io::Cursor;

use crate::db::db::ConfigMonkeyDb;
use crate::services::envs_svc::{self, EnvsServiceError};
use chrono::{DateTime, Utc};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::serde_json::to_string;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, Response};
use rocket_db_pools::Connection;

use super::dtos::{ErrorMessageDto, PaginationDto};

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

pub struct EnvsRoutesError(EnvsServiceError);

impl<'a> Responder<'a, 'static> for EnvsRoutesError {
    fn respond_to(self, _: &'a Request<'_>) -> rocket::response::Result<'static> {
        let EnvsRoutesError(apps_service_error) = self;
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
pub struct GetEnvsResponse(Json<GetEnvsDto>);

#[get("/v1/envs/<app_slug>?<limit>&<offset>")]
pub async fn get_envs(
    db: Connection<ConfigMonkeyDb>,
    app_slug: &str,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<GetEnvsResponse, EnvsRoutesError> {
    let result = envs_svc::get_envs(db, app_slug, limit, offset).await;
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
        Err(err) => Err(EnvsRoutesError(err)),
    };
}
