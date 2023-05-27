use crate::models::config::Config;
use crate::services::configs_svc;
use rocket::serde::json::Json;

#[get("/v1/configs")]
pub async fn get_configs() -> Json<Vec<Config>> {
    let configs = configs_svc::get_configs();
    Json(configs)
}
