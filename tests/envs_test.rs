use configmonkey::routes::v1::envs_routes::GetEnvDto;
use rocket::{
    http::{ContentType, Status},
    serde::json::serde_json::from_str,
};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::common::helpers::{async_client_from_pg_connect_options, h_create_app, h_create_env};

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
    let response_body = response.into_string().await.expect("Response Body");
    let env_dto: GetEnvDto = from_str(&response_body.as_str()).expect("Valid Env Dto");
    assert_eq!(env_dto.slug, "production");
    assert_eq!(env_dto.name, "Production");

    Ok(())
}
