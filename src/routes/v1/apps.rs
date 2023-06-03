use std::io::Cursor;

use crate::{
    db::db::ConfigMonkeyDb,
    services::apps_svc::{self, AppsServiceError},
};
use rocket::{
    http::{ContentType, Status},
    response::Responder,
    serde::{json::to_string, json::Json, Deserialize, Serialize},
    Request, Response,
};
use rocket_db_pools::Connection;

use super::common::ErrorMessageDto;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAppInput<'r> {
    slug: &'r str,
    name: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppDto {
    pub id: String,
    pub slug: String,
    pub name: String,
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetAppsResponse(Json<Vec<AppDto>>);

#[get("/v1/apps")]
pub async fn get_apps(db: Connection<ConfigMonkeyDb>) -> GetAppsResponse {
    let result = apps_svc::get_apps(db).await;
    let mut appdtos = vec![];

    return match result {
        Ok(apps) => {
            for app in apps {
                appdtos.push(AppDto {
                    name: app.name,
                    slug: app.slug,
                    id: app.id,
                });
            }
            GetAppsResponse(Json(appdtos))
        }
        Err(_e) => panic!("Panix !"),
    };
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreateAppSuccess(Json<AppDto>);

pub struct CreateAppError(AppsServiceError);

impl<'r> Responder<'r, 'static> for CreateAppError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let CreateAppError(apps_service_error) = self;
        let status = match apps_service_error {
            AppsServiceError::DuplicateSlug => Status::Conflict,
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

#[post("/v1/apps", format = "application/json", data = "<input>")]
pub async fn create_app(
    db: Connection<ConfigMonkeyDb>,
    input: Json<CreateAppInput<'_>>,
) -> Result<CreateAppSuccess, CreateAppError> {
    let result = apps_svc::create_app(db, String::from(input.slug), String::from(input.name)).await;

    return match result {
        Ok(app) => Ok(CreateAppSuccess(Json(AppDto {
            name: app.name,
            slug: app.slug,
            id: app.id,
        }))),
        Err(err) => Err(CreateAppError(err)),
    };
}
