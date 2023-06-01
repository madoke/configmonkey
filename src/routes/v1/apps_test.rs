use crate::routes::v1::apps::AppDto;

use super::super::super::rocket_from_config;
use rocket::http::ContentType;
use rocket::local::asynchronous::Client;
use rocket::serde::json::serde_json;
use rocket::{
    figment::{
        map,
        value::{Map, Value},
    },
    http::Status,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[sqlx::test]
fn happy_path_create_get_app(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let db_url = format!(
        "postgres://postgres:configmonkey@localhost:5432/{}",
        pg_connect_options.get_database().unwrap()
    );

    let db_config: Map<_, Value> = map! {
        "url" => db_url.into(),
    };

    let figment =
        rocket::Config::figment().merge(("databases", map!["postgres_configmonkey" => db_config]));

    let client = Client::tracked(rocket_from_config(figment))
        .await
        .expect("valid rocket instance");

    let response = client
        .post(uri!(crate::routes::v1::apps::create_app))
        .header(ContentType::JSON)
        .body(r#"{"slug": "connect","name": "Connect API"}"#)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.into_string().await.expect("Response Body");
    let app_dto: AppDto = serde_json::from_str(&response_body.as_str()).expect("Valid App Dto");
    assert_eq!(app_dto.name, "Connect API");
    assert_eq!(app_dto.slug, "connect");
    Ok(())
}
