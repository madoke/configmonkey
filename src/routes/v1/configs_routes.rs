use crate::db::db::ConfigMonkeyDb;
use crate::services::configs_service::{self, ConfigsServiceError};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::{json::Json, Deserialize, Serialize};

use rocket::post;
use rocket_db_pools::Connection;

use super::errors::RoutesError;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetConfigDto {
    pub key: String,
    pub version: i32,
    pub config_type: String,
    pub value_type: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateConfigDto {
    pub key: String,
    pub config_type: String,
    pub value_type: String,
    pub value: String,
}

fn to_http_status(error: &ConfigsServiceError) -> Status {
    match error {
        _ => Status::InternalServerError,
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
    let result = configs_service::create_config(
        db,
        domain_slug,
        input.key.as_str(),
        input.config_type.as_str(),
        input.value_type.as_str(),
        input.value.as_str(),
    )
    .await;

    match result {
        Ok(config) => Ok(CreateConfigSuccess(Json(GetConfigDto {
            key: config.key,
            version: config.version,
            config_type: config.config_type.to_string(),
            value_type: config.value_type.to_string(),
            value: config.value.to_string(),
            created_at: config.created_at,
            updated_at: config.updated_at,
        }))),
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
