use crate::{db::db::ConfigMonkeyDb, models::app::App};
use rocket_db_pools::{
    sqlx::{self, types::Uuid},
    Connection,
};

#[derive(sqlx::FromRow)]
struct AppEntity {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
}

pub async fn get_apps(mut db: Connection<ConfigMonkeyDb>) -> Result<Vec<App>, String> {
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
) -> Result<App, String> {
    let result = sqlx::query_as::<_, AppEntity>(
        "insert into apps(tenant, slug, name) values ('default', $1, $2) returning id, slug, name",
    )
    .bind(slug)
    .bind(name)
    .fetch_one(&mut *db)
    .await
    .unwrap();

    Ok(App {
        name: result.name,
        id: result.id.to_string(),
        slug: result.slug,
    })
}
