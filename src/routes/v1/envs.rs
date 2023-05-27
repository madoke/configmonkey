use crate::models::env::Env;
use crate::services::envs_svc;
use rocket::serde::json::Json;

#[get("/v1/envs")]
pub async fn get_envs() -> Json<Vec<Env>> {
    let envs = envs_svc::get_envs();
    Json(envs)
}
