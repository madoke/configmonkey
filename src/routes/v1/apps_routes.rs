use super::common::ErrorMessageDto;
use crate::{
    db::db::ConfigMonkeyDb,
    services::apps_svc::{self, AppsServiceError},
};
use chrono::{DateTime, Utc};
use rocket::{
    http::{ContentType, Status},
    response::Responder,
    serde::{json::to_string, json::Json, Deserialize, Serialize},
    Request, Response,
};
use rocket_db_pools::Connection;
use std::io::Cursor;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAppInput<'a> {
    slug: &'a str,
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PaginationDto<'a> {
    pub count: i32,
    pub offset: i32,
    pub limit: i32,
    pub next: &'a str,
    pub prev: &'a str,
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
pub struct GetAppsDto<'a> {
    pub data: Vec<GetAppDto>,
    #[serde(borrow = "'a")]
    pub pagination: PaginationDto<'a>,
}

pub struct AppsRoutesError(AppsServiceError);

impl<'a> Responder<'a, 'static> for AppsRoutesError {
    fn respond_to(self, _: &'a Request<'_>) -> rocket::response::Result<'static> {
        let AppsRoutesError(apps_service_error) = self;
        let status = match apps_service_error {
            AppsServiceError::DuplicateSlug => Status::Conflict,
            AppsServiceError::InvalidSlug => Status::BadRequest,
            AppsServiceError::InvalidName => Status::BadRequest,
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
pub struct GetAppsResponse<'a>(Json<GetAppsDto<'a>>);

#[get("/v1/apps")]
pub async fn get_apps(
    db: Connection<ConfigMonkeyDb>,
) -> Result<GetAppsResponse<'static>, AppsRoutesError> {
    let result = apps_svc::get_apps(db).await;
    let mut appdtos = vec![];

    return match result {
        Ok(apps) => {
            for app in apps {
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
                    count: 1,
                    offset: 1,
                    limit: 1,
                    next: "()",
                    prev: "()",
                },
            })))
        }
        Err(err) => Err(AppsRoutesError(err)),
    };
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreateAppSuccess(Json<GetAppDto>);

#[post("/v1/apps", format = "application/json", data = "<input>")]
pub async fn create_app(
    db: Connection<ConfigMonkeyDb>,
    input: Json<CreateAppInput<'_>>,
) -> Result<CreateAppSuccess, AppsRoutesError> {
    let result = apps_svc::create_app(db, input.slug, input.name).await;

    return match result {
        Ok(app) => Ok(CreateAppSuccess(Json(GetAppDto {
            name: app.name,
            slug: app.slug,
            id: app.id,
            created_at: app.created_at,
            updated_at: app.updated_at,
        }))),
        Err(err) => Err(AppsRoutesError(err)),
    };
}

#[derive(Responder)]
#[response(status = 204, content_type = "json")]
pub struct DeleteAppSuccess(());

#[delete("/v1/apps/<slug>")]
pub async fn delete_app(
    db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<DeleteAppSuccess, AppsRoutesError> {
    let result = apps_svc::delete_app(db, slug).await;

    return match result {
        Ok(()) => Ok(DeleteAppSuccess(())),
        Err(err) => Err(AppsRoutesError(err)),
    };
}
