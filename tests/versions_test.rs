use configmonkey::routes::v1::{configs_routes::GetVersionDto, dtos::{ErrorDto, PaginatedListDto}};
use rocket::{
    http::{ContentType, Status},
    serde::json::json,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod common;

pub use common::helpers::*;

#[sqlx::test]
async fn create_version_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    h_create_config(&client, "configmonkey", "database_url").await;

    let versions = vec![
        json!("postgres://localhost:1337"),
        json!(1.0),
        json!(1),
        json!(true),
    ];

    for version in versions.iter() {
        let response =
            h_create_version(&client, "configmonkey", "database_url", json!(version)).await;

        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let response_body = h_parse_response(response).await;
        let get_version_dto: GetVersionDto = h_parse_dto(response_body.as_str());
        assert_eq!(get_version_dto.value, json!(version));
    }

    Ok(())
}

#[sqlx::test]
async fn create_version_err_domain_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let response = h_create_version(
        &client,
        "configmonkey",
        "database_url",
        json!("postgres://localhost:1337"),
    )
    .await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "domain_not_found");
    assert_eq!(error_dto.message, "Domain not found");

    Ok(())
}

#[sqlx::test]
async fn create_version_err_config_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;

    let response = h_create_version(
        &client,
        "configmonkey",
        "database_url",
        json!("postgres://localhost:1337"),
    )
    .await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "config_not_found");
    assert_eq!(error_dto.message, "Config not found");

    Ok(())
}


#[sqlx::test]
async fn get_versions_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    h_create_config(&client, "configmonkey", "database_url").await;
    h_create_version(&client, "configmonkey", "database_url", json!("postgres://localhost:1337")).await;
    h_create_version(&client, "configmonkey", "database_url", json!("postgres://localhost:1338")).await;

    // get first page
    let response = h_get_versions(&client, "configmonkey", "database_url", Some(1), Some(0)).await;
    
    let response_body = h_parse_response(response).await;
    let get_versions_dto: PaginatedListDto<GetVersionDto> = h_parse_dto(response_body.as_str());

    assert_eq!(get_versions_dto.data.len(), 1);
    assert_eq!(get_versions_dto.data[0].value, "postgres://localhost:1338");

    h_validate_pagination(
        get_versions_dto.pagination,
        1,
        1,
        0,
        Some(String::from("/v1/configs/configmonkey/database_url/versions?limit=1&offset=1")),
        None,
    );

    // get second page
    let response = h_get_versions(&client, "configmonkey", "database_url", Some(1), Some(1)).await;

    let response_body = h_parse_response(response).await;
    let get_versions_dto: PaginatedListDto<GetVersionDto> = h_parse_dto(response_body.as_str());

    assert_eq!(get_versions_dto.data.len(), 1);
    assert_eq!(get_versions_dto.data[0].value, "postgres://localhost:1337");

    h_validate_pagination(
        get_versions_dto.pagination,
        1,
        1,
        1,
        Some(String::from("/v1/configs/configmonkey/database_url/versions?limit=1&offset=2")),
        Some(String::from("/v1/configs/configmonkey/database_url/versions?limit=1&offset=0")),
    );

    Ok(())
}


#[sqlx::test]
async fn get_versions_err_domain_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let response = h_get_versions(&client, "configmonkey", "database_url", Some(1), Some(0)).await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "domain_not_found");
    assert_eq!(error_dto.message, "Domain not found");

    Ok(())

}


#[sqlx::test]
async fn get_versions_err_config_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;

    let response = h_get_versions(&client, "configmonkey", "database_url", Some(1), Some(0)).await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "config_not_found");
    assert_eq!(error_dto.message, "Config not found");

    Ok(())

}

