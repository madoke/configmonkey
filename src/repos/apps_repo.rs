use std::borrow::Cow;

use crate::{db::db::ConfigMonkeyDb, models::app::App};
use rocket_db_pools::{
    sqlx::{self, types::Uuid},
    Connection,
};
use sqlx::Error;

pub enum AppsRepoError {
    DuplicateSlug,
}

#[derive(sqlx::FromRow)]
struct AppEntity {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
}

pub async fn get_apps(mut db: Connection<ConfigMonkeyDb>) -> Result<Vec<App>, AppsRepoError> {
    let result = sqlx::query_as::<_, AppEntity>("select id, slug, name from apps")
        .fetch_all(&mut *db)
        .await
        .unwrap();

    let mut apps = vec![];

    for app in result {
        apps.push(App {
            name: app.name,
            id: app.id.to_string(),
            slug: app.slug,
        })
    }

    Ok(apps)
}

pub async fn create_app(
    mut db: Connection<ConfigMonkeyDb>,
    slug: String,
    name: String,
) -> Result<App, AppsRepoError> {
    let result = sqlx::query_as::<_, AppEntity>(
        "insert into apps(tenant, slug, name) values ('default', $1, $2) returning id, slug, name",
    )
    .bind(slug)
    .bind(name)
    .fetch_one(&mut *db)
    .await;

    match result {
        Ok(entity) => Ok(App {
            name: entity.name,
            id: entity.id.to_string(),
            slug: entity.slug,
        }),
        Err(Error::Database(err)) => match err.code() {
            // Postgres code for unique_violation: https://www.postgresql.org/docs/current/errcodes-appendix.html
            Some(Cow::Borrowed("23505")) => Err(AppsRepoError::DuplicateSlug),
            // TODO: Convert into log + Unknown error instead of panic
            _ => panic!("{}", err.to_string()),
        },
        // TODO: Convert into log + Unknown error instead of panic
        Err(err) => panic!("{}", err.to_string()),
    }
}
