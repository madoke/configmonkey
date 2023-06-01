use db::db::{run_migrations, ConfigMonkeyDb};
use rocket::{fairing::AdHoc, figment::Figment, Build, Config, Rocket};
use rocket_db_pools::Database;

#[macro_use]
extern crate rocket;

mod db;
mod models;
mod repos;
mod routes;
mod services;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket_from_config(Config::figment())
}

fn rocket_from_config(figment: Figment) -> Rocket<Build> {
    rocket::custom(figment)
        .attach(ConfigMonkeyDb::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount(
            "/",
            routes![
                routes::v1::apps::get_apps,
                routes::v1::apps::create_app,
                routes::v1::envs::get_envs,
                routes::v1::configs::get_configs
            ],
        )
}
