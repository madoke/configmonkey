use crate::{db::db::ConfigMonkeyDb, models::app::App, services};
use rocket::{serde::json::Json, serde::Deserialize};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection,
};

#[get("/v1/apps")]
pub async fn get_apps() -> Json<Vec<App>> {
    let apps = services::apps_svc::get_apps();
    Json(apps)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAppInput<'r> {
    slug: &'r str,
}

#[post("/v1/apps", data = "<input>")]
pub async fn create_app(input: Json<CreateAppInput<'_>>) {
    services::apps_svc::create_app(String::from(input.slug));
}

#[get("/v1/apps/test")]
pub async fn read(mut db: Connection<ConfigMonkeyDb>) -> Option<String> {
    sqlx::query("SELECT 'abc'")
        .fetch_one(&mut *db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}
