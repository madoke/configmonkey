use crate::routes::v1::apps_routes::AppDto;
use crate::routes::v1::common::ErrorMessageDto;
use crate::test::async_client_from_pg_connect_options;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::local::asynchronous::LocalResponse;
use rocket::serde::json::serde_json;
use rocket::serde::json::serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

// helpers

async fn h_create_app<'a>(client: &'a Client, app_slug: &str, app_name: &str) -> LocalResponse<'a> {
    client
        .post(uri!(crate::routes::v1::apps_routes::create_app))
        .header(ContentType::JSON)
        .body(json!({"slug": app_slug, "name": app_name }).to_string())
        .dispatch()
        .await
}

async fn h_get_apps<'a>(client: &'a Client) -> Vec<AppDto> {
    let response = client
        .get(uri!(crate::routes::v1::apps_routes::get_apps))
        .dispatch()
        .await;

    // assert response
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = response.into_string().await.expect("Response Body");
    let apps: Vec<AppDto> = serde_json::from_str(&response_body.as_str()).expect("Valid App List");
    apps
}

// test cases

#[sqlx::test]
async fn create_app_success(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app
    let response = h_create_app(&client, "configmonkey", "Config Monkey").await;

    // assert response
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // assert body
    let response_body = response.into_string().await.expect("Response Body");
    let app_dto: AppDto = serde_json::from_str(&response_body.as_str()).expect("Valid App Dto");
    assert_eq!(app_dto.slug, "configmonkey");
    assert_eq!(app_dto.name, "Config Monkey");

    Ok(())
}

#[sqlx::test]
async fn get_apps_success(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app
    h_create_app(&client, "configmonkey", "Config Monkey").await;

    // get the list of apps
    let apps = h_get_apps(&client).await;

    //assert apps
    assert_eq!(apps.len(), 1);
    assert_eq!(apps[0].slug, "configmonkey");
    assert_eq!(apps[0].name, "Config Monkey");

    Ok(())
}

#[sqlx::test]
async fn err_duplicate_slug(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create duplicate apps
    h_create_app(&client, "configmonkey", "Config Monkey").await;
    let response = h_create_app(&client, "configmonkey", "Config Monkey").await;

    // assert response
    assert_eq!(response.status(), Status::Conflict);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    //assert body
    let response_body = response.into_string().await.expect("Response Body");
    let error_message: ErrorMessageDto =
        serde_json::from_str(&response_body.as_str()).expect("Valid Error Message");
    assert_eq!(error_message.code, "duplicate_slug");
    assert_eq!(
        error_message.message,
        "An app with the same slug already exists"
    );

    Ok(())
}

#[sqlx::test]
async fn delete_app_success(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create duplicate apps
    h_create_app(&client, "configmonkey", "Config Monkey").await;

    // delete app
    let response = client
        .delete(uri!(crate::routes::v1::apps_routes::delete_app(
            "configmonkey"
        )))
        .dispatch()
        .await;

    // assert response
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // get the list of apps
    let apps = h_get_apps(&client).await;

    //assert apps
    assert_eq!(apps.len(), 0);

    Ok(())
}
