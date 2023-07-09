use super::routes;
use crate::db::db::{run_migrations, ConfigMonkeyDb};
use rocket::{catchers, fairing::AdHoc, figment::Figment, routes, Build, Rocket};
use rocket_db_pools::Database;

pub fn rocket_from_config(figment: Figment) -> Rocket<Build> {
    rocket::custom(figment)
        .attach(ConfigMonkeyDb::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount(
            "/",
            routes![
                routes::v1::apps_routes::get_apps,
                routes::v1::apps_routes::create_app,
                routes::v1::apps_routes::delete_app,
                routes::v1::envs_routes::get_envs,
                routes::v1::envs_routes::create_env,
                routes::v1::envs_routes::delete_env,
                routes::v1::configs_routes::create_config,
                routes::v1::configs_routes::get_config,
                routes::v1::configs_routes::delete_config,
            ],
        )
        .register(
            "/",
            catchers![
                routes::v1::errors::default_catcher,
                routes::v1::errors::not_found
            ],
        )
}
