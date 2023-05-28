use db::db::{run_migrations, ConfigMonkeyDb};
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;
mod services;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(ConfigMonkeyDb::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount(
            "/",
            routes![
                routes::v1::apps::read,
                routes::v1::apps::get_apps,
                routes::v1::apps::create_app,
                routes::v1::envs::get_envs,
                routes::v1::configs::get_configs
            ],
        )
}
