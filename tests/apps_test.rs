use configmonkey::{
    self,
    routes::v1::{
        apps_routes::{rocket_uri_macro_delete_app, GetAppDto},
        dtos::ErrorMessageDto,
    },
};
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::uri;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod common;

pub use common::helpers::*;

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
async fn create_app_err_duplicate_slug(
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
async fn create_app_err_invalid_slug(
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
        // create app with bad slug
        let response = h_create_app(&client, &bad_slug, "Config Monkey").await;

        // assert response
        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        //assert body
        let error_message_dto = h_parse_error(response).await;
        assert_eq!(error_message_dto.code, "invalid_slug");
        assert_eq!(error_message_dto.message, "The slug contains invalid characters. Only lowercase letters, numbers and dash (-) are allowed");
    }

    Ok(())
}

#[sqlx::test]
async fn create_app_err_invalid_name(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let bad_names = vec!["[ConfigMonkey]", "Config --- Monkey     ", "config monkey "];

    for bad_name in bad_names.iter() {
        // create app with bad name
        let response = h_create_app(&client, "configmonkey", &bad_name).await;

        // assert response
        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        //assert body
        let error_message_dto = h_parse_error(response).await;
        assert_eq!(error_message_dto.code, "invalid_name");
        assert_eq!(error_message_dto.message, "The name contains invalid characters. Only letters, numbers, spaces and underscore (_) are allowed");
    }

    Ok(())
}

#[sqlx::test]
async fn delete_app_success(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app
    h_create_app(&client, "configmonkey", "Config Monkey").await;

    // delete app
    let response = client
        .delete(uri!(delete_app("configmonkey")))
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
