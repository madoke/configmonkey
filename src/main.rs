use rocket::{Build, Config, Rocket};

#[macro_use]
extern crate rocket;

mod app;
mod db;
mod models;
mod repos;
mod routes;
mod services;
mod shared;

#[launch]
fn rocket() -> Rocket<Build> {
    app::rocket_from_config(Config::figment())
}
