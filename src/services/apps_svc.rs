use crate::{db::db::ConfigMonkeyDb, models::app::App, repos::apps_repo};
use rocket_db_pools::Connection;

pub async fn get_apps(db: Connection<ConfigMonkeyDb>) -> Result<Vec<App>, String> {
    return apps_repo::get_apps(db).await;
}

pub async fn create_app(
    db: Connection<ConfigMonkeyDb>,
    slug: String,
    name: String,
) -> Result<App, String> {
    return apps_repo::create_app(db, slug, name).await;
}
