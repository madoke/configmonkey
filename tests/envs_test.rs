use rocket::{
    http::{ContentType, Status},
    serde::json::serde_json::json,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod common;

pub use common::helpers::*;

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
async fn create_env_err_invalid_slug(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app
    h_create_app(&client, "configmonkey", "Config Monkey").await;

    let bad_slugs = vec!["Pr0dUc71oN", "!@#$%^&*(){}[]:;,prod", "Prod env"];

    for bad_slug in bad_slugs.iter() {
        // create env with bad slug
        let response = h_create_env(&client, "configmonkey", &bad_slug, "Production").await;

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
async fn create_env_err_invalid_name(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app
    h_create_app(&client, "configmonkey", "Config Monkey").await;

    let bad_names = vec!["[Production]", "Prod --- Env     ", "prod env "];

    for bad_name in bad_names.iter() {
        // create env with bad name
        let response = h_create_env(&client, "configmonkey", "production", &bad_name).await;

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

#[sqlx::test]
async fn delete_env_err_not_found(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app
    h_create_app(&client, "configmonkey", "Config Monkey").await;

    // delete env
    let response = h_delete_env(&client, "configmonkey", "env-not-exists").await;

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
async fn delete_env_err_app_not_found(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // delete env
    let response = h_delete_env(&client, "app-not-exists", "env-not-exists").await;

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
async fn delete_env_err_has_configs(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    // create app, env and config
    h_create_app(&client, "configmonkey", "Config Monkey").await;
    h_create_env(&client, "configmonkey", "production", "Production").await;
    h_create_config(
        &client,
        "configmonkey",
        "production",
        json!({"key":"value"}),
    )
    .await;

    // delete env
    let response = h_delete_env(&client, "configmonkey", "production").await;

    // assert response
    assert_eq!(response.status(), Status::UnprocessableEntity);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // assert body
    let error_message_dto = h_parse_error(response).await;
    assert_eq!(error_message_dto.code, "config_exists");
    assert_eq!(
        error_message_dto.message,
        "Cannot delete environment with existing config"
    );

    Ok(())
}
