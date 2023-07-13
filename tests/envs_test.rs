use rocket::http::{ContentType, Status};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::common::helpers::{
    async_client_from_pg_connect_options, h_create_app, h_create_env, h_delete_env, h_get_envs,
    h_parse_error, h_parse_get_env, h_parse_get_envs,
};

mod common;

// test cases

#[sqlx::test]
async fn create_env_success(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app
    h_create_app(&client, "configmonkey", "Config Monkey").await;

    // create env
    let response = h_create_env(&client, "configmonkey", "production", "Production").await;

    // assert response
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // assert body
    let get_env_dto = h_parse_get_env(response).await;
    assert_eq!(get_env_dto.slug, "production");
    assert_eq!(get_env_dto.name, "Production");

    Ok(())
}

#[sqlx::test]
async fn create_env_err_duplicate_env(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app and env
    h_create_app(&client, "configmonkey", "Config Monkey").await;
    h_create_env(&client, "configmonkey", "production", "Production").await;

    // create duplicate env
    let response = h_create_env(&client, "configmonkey", "production", "Production").await;

    // assert response
    assert_eq!(response.status(), Status::Conflict);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // assert body
    let error_message_dto = h_parse_error(response).await;
    assert_eq!(error_message_dto.code, "duplicate_slug");
    assert_eq!(
        error_message_dto.message,
        "An env with the same slug already exists"
    );

    Ok(())
}

#[sqlx::test]
async fn create_env_err_app_not_found(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app and env
    h_create_app(&client, "configmonkey", "Config Monkey").await;

    // create duplicate env
    let response = h_create_env(&client, "configninja", "production", "Production").await;

    // assert response
    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // assert body
    let error_message_dto = h_parse_error(response).await;
    assert_eq!(error_message_dto.code, "resource_not_found");
    assert_eq!(error_message_dto.message, "Resource not found");

    Ok(())
}

#[sqlx::test]
async fn delete_env_success(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app and env
    h_create_app(&client, "configmonkey", "Config Monkey").await;
    h_create_env(&client, "configmonkey", "production", "Production").await;

    // delete env
    let response = h_delete_env(&client, "configmonkey", "production").await;

    // assert response
    assert_eq!(response.status(), Status::NoContent);

    // assert no envs
    let get_envs_response = h_get_envs(&client, "configmonkey", None, None).await;
    let get_envs_dto = h_parse_get_envs(get_envs_response).await;
    assert_eq!(get_envs_dto.data.len(), 0);
    assert_eq!(get_envs_dto.pagination.count, 0);

    Ok(())
}
