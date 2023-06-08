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
                routes::v1::apps_routes::get_apps,
                routes::v1::apps_routes::create_app,
                routes::v1::apps_routes::delete_app,
                routes::v1::envs::get_envs,
                routes::v1::configs::get_configs
            ],
        )
}

#[cfg(test)]
pub mod test {
    use super::rocket_from_config;
    use rocket::{
        figment::{
            map,
            value::{Map, Value},
        },
        local::asynchronous::Client,
    };
    use sqlx::postgres::PgConnectOptions;

    pub async fn async_client_from_pg_connect_options(
        pg_connect_options: PgConnectOptions,
    ) -> Client {
        let db_url = format!(
            "postgres://postgres:configmonkey@localhost:5432/{}",
            pg_connect_options.get_database().unwrap()
        );

        let db_config: Map<_, Value> = map! {
            "url" => db_url.into(),
        };

        let figment = rocket::Config::figment()
            .merge(("databases", map!["postgres_configmonkey" => db_config]));

        let client = Client::tracked(rocket_from_config(figment))
            .await
            .expect("valid rocket instance");

        return client;
    }
}
