use rocket_db_pools::sqlx;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("postgres_configmonkey")]
pub struct ConfigMonkeyDb(sqlx::PgPool);
