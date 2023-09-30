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
                routes::v1::domains_routes::create_domain,
                routes::v1::domains_routes::get_domains,
                routes::v1::domains_routes::delete_domain,
                routes::v1::configs_routes::create_config,
                routes::v1::configs_routes::get_configs,
                routes::v1::configs_routes::get_config,
                routes::v1::configs_routes::delete_config,
                routes::v1::versions_routes::create_version,
                routes::v1::versions_routes::get_versions,
            ],
        )
        .register(
            "/",
            catchers![
                routes::v1::errors::default_catcher,
                routes::v1::errors::not_found,
                routes::v1::errors::bad_request
            ],
        )
}
