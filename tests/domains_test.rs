use configmonkey::routes::v1::{
    domains_routes::GetDomainDto,
    dtos::{ErrorDto, PaginatedListDto},
};
use rocket::http::{ContentType, Status};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

mod common;
pub use common::helpers::*;

#[sqlx::test]
async fn create_domain_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let response = h_create_domain(&client, "configmonkey").await;

    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let domain_dto: GetDomainDto = h_parse_dto(response_body.as_str());
    assert_eq!(domain_dto.slug, "configmonkey");

    Ok(())
}

#[sqlx::test]
async fn get_domains_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    h_create_domain(&client, "configchimp").await;

    // get first page
    let response = h_get_domains(&client, Some(1), Some(0)).await;
    let response_body = h_parse_response(response).await;
    let get_domains_dto: PaginatedListDto<GetDomainDto> = h_parse_dto(response_body.as_str());

    assert_eq!(get_domains_dto.data.len(), 1);
    assert_eq!(get_domains_dto.data[0].slug, "configmonkey");

    h_validate_pagination(
        get_domains_dto.pagination,
        1,
        1,
        0,
        Some(String::from("/v1/domains?limit=1&offset=1")),
        None,
    );

    // get second page
    let response = h_get_domains(&client, Some(1), Some(1)).await;
    let response_body = h_parse_response(response).await;
    let get_domains_dto: PaginatedListDto<GetDomainDto> = h_parse_dto(response_body.as_str());

    assert_eq!(get_domains_dto.data.len(), 1);
    assert_eq!(get_domains_dto.data[0].slug, "configchimp");

    h_validate_pagination(
        get_domains_dto.pagination,
        1,
        1,
        1,
        Some(String::from("/v1/domains?limit=1&offset=2")),
        Some(String::from("/v1/domains?limit=1&offset=0")),
    );

    Ok(())
}

#[sqlx::test]
async fn create_domain_err_duplicate_slug(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    let response = h_create_domain(&client, "configmonkey").await;

    assert_eq!(response.status(), Status::Conflict);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "duplicate_slug");
    assert_eq!(
        error_dto.message,
        "A domain with the same slug already exists"
    );

    Ok(())
}

#[sqlx::test]
async fn create_domain_err_invalid_slug(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let bad_slugs = vec![
        "Config Monkey",
        "!@#$%^&*(){}[]:;,configmonkey",
        "config/monkey",
    ];

    for bad_slug in bad_slugs.iter() {
        let response = h_create_domain(&client, &bad_slug).await;

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let response_body = h_parse_response(response).await;
        let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
        assert_eq!(error_dto.code, "invalid_slug");
        assert_eq!(error_dto.message, "The slug contains invalid characters. Only letters, numbers, dash (-) and underscore (_) are allowed");
    }

    Ok(())
}

#[sqlx::test]
async fn delete_domain_success(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;

    let response = h_delete_domain(&client, "configmonkey").await;

    assert_eq!(response.status(), Status::NoContent);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response = h_get_domains(&client, None, None).await;
    let response_body = h_parse_response(response).await;
    let get_domains_dto: PaginatedListDto<GetDomainDto> = h_parse_dto(response_body.as_str());

    assert_eq!(get_domains_dto.data.len(), 0);
    assert_eq!(get_domains_dto.pagination.count, 0);

    Ok(())
}

#[sqlx::test]
async fn delete_domain_err_not_found(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    let response = h_delete_domain(&client, "configmonkey").await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "not_found");
    assert_eq!(error_dto.message, "Domain not found");

    Ok(())
}

#[sqlx::test]
async fn delete_domain_err_not_empty(
    _: PgPoolOptions,
    pg_connect_options: PgConnectOptions,
) -> sqlx::Result<()> {
    let client = async_client_from_pg_connect_options(pg_connect_options).await;

    h_create_domain(&client, "configmonkey").await;
    h_create_config(&client, "configmonkey", "database_url").await;

    let response = h_delete_domain(&client, "configmonkey").await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let response_body = h_parse_response(response).await;
    let error_dto: ErrorDto = h_parse_dto(response_body.as_str());
    assert_eq!(error_dto.code, "not_empty");
    assert_eq!(
        error_dto.message,
        "The domain could not be deleted because there are existing configs"
    );

    Ok(())
}
