use configmonkey::routes::v1::{
    configs_routes::GetConfigDto,
    dtos::{ErrorDto, PaginatedListDto},
};
use rocket::http::{ContentType, Status};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod common;

pub use common::helpers::*;

#[sqlx::test]
async fn create_config_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;

    let response = h_create_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let get_config_dto: GetConfigDto = h_parse_dto(response_body.as_str());
    assert_eq!(get_config_dto.key, "database_url");

    Ok(())
}

#[sqlx::test]
async fn create_config_err_exists(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    h_create_config(&client, "configmonkey", "database_url").await;

    let response = h_create_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::Conflict);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "config_already_exists");
    assert_eq!(error_dto.message, "Config already exists");

    Ok(())
}

#[sqlx::test]
async fn create_config_err_domain_not_found(
    _pg_pool_options: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let response = h_create_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "domain_not_found");
    assert_eq!(error_dto.message, "Domain not found");

    Ok(())
}

#[sqlx::test]
async fn get_configs_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    h_create_config(&client, "configmonkey", "database_url").await;
    h_create_config(&client, "configmonkey", "database_port").await;

    // get first page
    let response = h_get_configs(&client, "configmonkey", Some(1), Some(0)).await;
    let response_body = h_parse_response(response).await;
    let get_configs_dto: PaginatedListDto<GetConfigDto> = h_parse_dto(response_body.as_str());

    assert_eq!(get_configs_dto.data.len(), 1);
    assert_eq!(get_configs_dto.data[0].key, "database_port");

    h_validate_pagination(
        get_configs_dto.pagination,
        1,
        1,
        0,
        Some(String::from("/v1/configs/configmonkey?limit=1&offset=1")),
        None,
    );

    // get second page
    let response = h_get_configs(&client, "configmonkey", Some(1), Some(1)).await;
    let response_body = h_parse_response(response).await;
    let get_configs_dto: PaginatedListDto<GetConfigDto> = h_parse_dto(response_body.as_str());

    assert_eq!(get_configs_dto.data.len(), 1);
    assert_eq!(get_configs_dto.data[0].key, "database_url");

    h_validate_pagination(
        get_configs_dto.pagination,
        1,
        1,
        1,
        Some(String::from("/v1/configs/configmonkey?limit=1&offset=2")),
        Some(String::from("/v1/configs/configmonkey?limit=1&offset=0")),
    );

    Ok(())
}

#[sqlx::test]
async fn get_config_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    h_create_config(&client, "configmonkey", "database_url").await;

    let response = h_get_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let get_config_dto: GetConfigDto = h_parse_dto(response_body.as_str());
    assert_eq!(get_config_dto.key, "database_url");

    Ok(())
}

#[sqlx::test]
async fn get_config_err_domain_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let response = h_get_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "domain_not_found");
    assert_eq!(error_dto.message, "Domain not found");

    Ok(())
}

#[sqlx::test]
async fn get_config_err_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;

    let response = h_get_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "config_not_found");
    assert_eq!(error_dto.message, "Config not found");

    Ok(())
}

#[sqlx::test]
async fn delete_config_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    h_create_config(&client, "configmonkey", "database_url").await;

    let response = h_get_config(&client, "configmonkey", "database_url").await;

    // assert exists
    assert_eq!(response.status(), Status::Ok);

    // delete config
    let response = h_delete_config(&client, "configmonkey", "database_url").await;

    // assert response
    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    // assert not exists
    let response = h_get_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::NotFound);

    Ok(())
}

#[sqlx::test]
async fn delete_config_err_domain_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let response = h_delete_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "domain_not_found");
    assert_eq!(error_dto.message, "Domain not found");

    Ok(())
}

#[sqlx::test]
async fn delete_config_err_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;
    h_create_domain(&client, "configmonkey").await;

    let response = h_delete_config(&client, "configmonkey", "database_url").await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "config_not_found");
    assert_eq!(error_dto.message, "Config not found");

    Ok(())
}
