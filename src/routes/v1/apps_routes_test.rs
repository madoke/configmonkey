use crate::routes::v1::apps_routes::GetAppDto;
use crate::routes::v1::common::ErrorMessageDto;
use crate::test::async_client_from_pg_connect_options;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::local::asynchronous::LocalResponse;
use rocket::serde::json::serde_json;
use rocket::serde::json::serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use super::apps_routes::GetAppsDto;

// helpers

async fn h_create_app<'a>(client: &'a Client, app_slug: &str, app_name: &str) -> LocalResponse<'a> {
    client
        .post(uri!(crate::routes::v1::apps_routes::create_app))
        .header(ContentType::JSON)
        .body(json!({"slug": app_slug, "name": app_name }).to_string())
        .dispatch()
        .await
}

async fn h_get_apps<'a>(client: &'a Client, limit: Option<i32>, offset: Option<i32>) -> GetAppsDto {
    let response = client
        .get(uri!(crate::routes::v1::apps_routes::get_apps(
            limit, offset
        )))
        .dispatch()
        .await;

    // assert response
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = response.into_string().await.expect("Response Body");
    let get_apps_dto: GetAppsDto =
        serde_json::from_str(&response_body.as_str()).expect("Valid Get Apps Response");
    get_apps_dto
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
    let app_dto: GetAppDto = serde_json::from_str(&response_body.as_str()).expect("Valid App Dto");
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
    h_create_app(&client, "configmonkey1", "Config Monkey 1").await;
    h_create_app(&client, "configmonkey2", "Config Monkey 2").await;

    // get first page
    let get_apps_dto = h_get_apps(&client, Some(1), Some(0)).await;

    //assert apps
    assert_eq!(get_apps_dto.data.len(), 1);
    assert_eq!(get_apps_dto.data[0].slug, "configmonkey1");
    assert_eq!(get_apps_dto.data[0].name, "Config Monkey 1");

    //assert pagination
    assert_eq!(get_apps_dto.pagination.count, 1);
    assert_eq!(get_apps_dto.pagination.limit, 1);
    assert_eq!(get_apps_dto.pagination.offset, 0);
    assert_eq!(
        get_apps_dto.pagination.next.unwrap(),
        "/v1/apps?limit=1&offset=1"
    );
    assert!(get_apps_dto.pagination.prev.is_none());

    // get second page
    let get_apps_dto = h_get_apps(&client, Some(1), Some(1)).await;

    //assert apps
    assert_eq!(get_apps_dto.data.len(), 1);
    assert_eq!(get_apps_dto.data[0].slug, "configmonkey2");
    assert_eq!(get_apps_dto.data[0].name, "Config Monkey 2");

    //assert pagination
    assert_eq!(get_apps_dto.pagination.count, 1);
    assert_eq!(get_apps_dto.pagination.limit, 1);
    assert_eq!(get_apps_dto.pagination.offset, 1);
    assert_eq!(
        get_apps_dto.pagination.next.unwrap(),
        "/v1/apps?limit=1&offset=2"
    );
    assert_eq!(
        get_apps_dto.pagination.prev.unwrap(),
        "/v1/apps?limit=1&offset=0"
    );

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
async fn err_invalid_slug(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let bad_slugs = vec![
        "CoNfIgMoNkEy",
        "!@#$%^&*(){}[]:;,configmonkey",
        "config monkey",
    ];

    for bad_slug in bad_slugs.iter() {
        // create duplicate apps
        let response = h_create_app(&client, &bad_slug, "Config Monkey").await;

        // assert response
        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        //assert body
        let response_body = response.into_string().await.expect("Response Body");
        let error_message: ErrorMessageDto =
            serde_json::from_str(&response_body.as_str()).expect("Valid Error Message");
        assert_eq!(error_message.code, "invalid_slug");
        assert_eq!(
            error_message.message,
            "The slug contains invalid characters. Only lowercase letters, numbers and dash (-) are allowed"
        );
    }

    Ok(())
}

#[sqlx::test]
async fn err_invalid_name(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let bad_names = vec!["[ConfigMonkey]", "Config --- Monkey     ", "config monkey "];

    for bad_name in bad_names.iter() {
        // create duplicate apps
        let response = h_create_app(&client, "configmonkey", &bad_name).await;

        // assert response
        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        //assert body
        let response_body = response.into_string().await.expect("Response Body");
        let error_message: ErrorMessageDto =
            serde_json::from_str(&response_body.as_str()).expect("Valid Error Message");
        assert_eq!(error_message.code, "invalid_name");
        assert_eq!(
            error_message.message,
            "The name contains invalid characters. Only letters, numbers, spaces and underscore (_) are allowed"
        );
    }

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
    let get_apps_dto = h_get_apps(&client, None, None).await;

    // assert no apps
    assert_eq!(get_apps_dto.data.len(), 0);
    assert_eq!(get_apps_dto.pagination.count, 0);

    Ok(())
}
