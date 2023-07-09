use rocket::{error, fairing, Build, Rocket};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres_configmonkey")]
pub struct ConfigMonkeyDb(sqlx::PgPool);

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match ConfigMonkeyDb::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./src/db/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}
