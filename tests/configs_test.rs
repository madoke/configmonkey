// use rocket::{
//     http::{ContentType, Status},
//     serde::json::serde_json::json,
// };
// use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

// mod common;

// pub use common::helpers::*;

// // test cases

// #[sqlx::test]
// async fn create_config_success(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // create app and env
//     h_create_app(&client, "configmonkey", "Config Monkey").await;
//     h_create_env(&client, "configmonkey", "staging", "Staging").await;

//     // create config
//     let response = h_create_config(
//         &client,
//         "configmonkey",
//         "staging",
//         json!({"key":"value"}).to_string().as_str(),
//     )
//     .await;

//     // assert response
//     assert_eq!(response.status(), Status::Created);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let get_config_dto = h_parse_get_config(response).await;
//     assert_eq!(get_config_dto.config, json!({"key":"value"}));

//     Ok(())
// }

// #[sqlx::test]
// async fn create_config_err_exists(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // create app, env and config
//     h_create_app(&client, "configmonkey", "Config Monkey").await;
//     h_create_env(&client, "configmonkey", "staging", "Staging").await;
//     h_create_config(
//         &client,
//         "configmonkey",
//         "staging",
//         json!({"key":"value"}).to_string().as_str(),
//     )
//     .await;

//     // create another config
//     let response = h_create_config(
//         &client,
//         "configmonkey",
//         "staging",
//         json!({"key":"value"}).to_string().as_str(),
//     )
//     .await;

//     // assert response
//     assert_eq!(response.status(), Status::Conflict);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let error_message_dto = h_parse_error(response).await;
//     assert_eq!(error_message_dto.code, "config_already_exists");
//     assert_eq!(error_message_dto.message, "Config already exists");

//     Ok(())
// }

// #[sqlx::test]
// async fn create_config_err_not_found(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // create config
//     let response = h_create_config(
//         &client,
//         "configmonkey",
//         "staging",
//         json!({"key":"value"}).to_string().as_str(),
//     )
//     .await;

//     // assert response
//     assert_eq!(response.status(), Status::NotFound);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let error_message_dto = h_parse_error(response).await;
//     assert_eq!(error_message_dto.code, "resource_not_found");
//     assert_eq!(error_message_dto.message, "Resource not found");

//     Ok(())
// }

// #[sqlx::test]
// async fn create_config_err_malformed(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // create app and env
//     h_create_app(&client, "configmonkey", "Config Monkey").await;
//     h_create_env(&client, "configmonkey", "staging", "Staging").await;

//     // create config
//     let response = h_create_config(&client, "configmonkey", "staging", "{\"syntax':error}}").await;

//     // assert response
//     assert_eq!(response.status(), Status::BadRequest);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let error_message_dto = h_parse_error(response).await;
//     assert_eq!(error_message_dto.code, "invalid_config_format");
//     assert_eq!(
//         error_message_dto.message,
//         "Invalid config format. Check the payload"
//     );

//     Ok(())
// }

// #[sqlx::test]
// async fn get_config_success(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // create app, env and config
//     h_create_app(&client, "configmonkey", "Config Monkey").await;
//     h_create_env(&client, "configmonkey", "staging", "Staging").await;
//     h_create_config(
//         &client,
//         "configmonkey",
//         "staging",
//         json!({"key":"value"}).to_string().as_str(),
//     )
//     .await;

//     // get config
//     let response = h_get_config(&client, "configmonkey", "staging").await;

//     // assert response
//     assert_eq!(response.status(), Status::Ok);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let get_config_dto = h_parse_get_config(response).await;
//     assert_eq!(get_config_dto.config, json!({"key":"value"}));

//     Ok(())
// }

// #[sqlx::test]
// async fn get_config_err_not_exists(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // create app, env and config
//     h_create_app(&client, "configmonkey", "Config Monkey").await;
//     h_create_env(&client, "configmonkey", "staging", "Staging").await;

//     // get config
//     let response = h_get_config(&client, "configmonkey", "staging").await;

//     // assert response
//     assert_eq!(response.status(), Status::NotFound);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let error_message_dto = h_parse_error(response).await;
//     assert_eq!(error_message_dto.code, "resource_not_found");
//     assert_eq!(error_message_dto.message, "Resource not found");

//     Ok(())
// }

// #[sqlx::test]
// async fn get_config_err_app_env_not_exists(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // get config
//     let response = h_get_config(&client, "configmonkey", "staging").await;

//     // assert response
//     assert_eq!(response.status(), Status::NotFound);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let error_message_dto = h_parse_error(response).await;
//     assert_eq!(error_message_dto.code, "resource_not_found");
//     assert_eq!(error_message_dto.message, "Resource not found");

//     Ok(())
// }

// #[sqlx::test]
// async fn delete_config_success(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // create app, env and config
//     h_create_app(&client, "configmonkey", "Config Monkey").await;
//     h_create_env(&client, "configmonkey", "staging", "Staging").await;
//     h_create_config(
//         &client,
//         "configmonkey",
//         "staging",
//         json!({"key":"value"}).to_string().as_str(),
//     )
//     .await;

//     // delete config
//     let response = h_delete_config(&client, "configmonkey", "staging").await;

//     // assert response
//     assert_eq!(response.status(), Status::NoContent);

//     // get config
//     let response = h_get_config(&client, "configmonkey", "staging").await;

//     // assert response
//     assert_eq!(response.status(), Status::NotFound);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let error_message_dto = h_parse_error(response).await;
//     assert_eq!(error_message_dto.code, "resource_not_found");
//     assert_eq!(error_message_dto.message, "Resource not found");

//     Ok(())
// }

// #[sqlx::test]
// async fn delete_config_err_not_exists(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // create app and env
//     h_create_app(&client, "configmonkey", "Config Monkey").await;
//     h_create_env(&client, "configmonkey", "staging", "Staging").await;

//     // delete config
//     let response = h_delete_config(&client, "configmonkey", "staging").await;

//     // assert response
//     assert_eq!(response.status(), Status::NotFound);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let error_message_dto = h_parse_error(response).await;
//     assert_eq!(error_message_dto.code, "resource_not_found");
//     assert_eq!(error_message_dto.message, "Resource not found");

//     Ok(())
// }

// #[sqlx::test]
// async fn delete_config_err_app_env_not_exists(
//     _pg_pool_options: PgPoolOptions,
//     pg_connect_options: PgConnectOptions,
// ) -> sqlx::Result<()> {
//     let client = async_client_from_pg_connect_options(pg_connect_options).await;

//     // delete config
//     let response = h_delete_config(&client, "configmonkey", "staging").await;

//     // assert response
//     assert_eq!(response.status(), Status::NotFound);
//     assert_eq!(response.content_type(), Some(ContentType::JSON));

//     // assert body
//     let error_message_dto = h_parse_error(response).await;
//     assert_eq!(error_message_dto.code, "resource_not_found");
//     assert_eq!(error_message_dto.message, "Resource not found");

//     Ok(())
// }
