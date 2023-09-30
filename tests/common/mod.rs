#[cfg(test)]
pub mod helpers {
    use configmonkey::{
        app::rocket_from_config,
        routes::v1::{
            configs_routes::{
                rocket_uri_macro_create_config, rocket_uri_macro_delete_config,
                rocket_uri_macro_get_config, rocket_uri_macro_get_configs,
            },
            domains_routes::{
                rocket_uri_macro_create_domain, rocket_uri_macro_delete_domain,
                rocket_uri_macro_get_domains,
            },
            dtos::PaginationDto,
            versions_routes::{rocket_uri_macro_create_version, rocket_uri_macro_get_versions},
        },
    };
    use rocket::{
        figment::{
            map,
            value::{Map, Value},
        },
        http::ContentType,
        local::asynchronous::{Client, LocalResponse},
        serde::{
            json::{from_str, serde_json::json},
            Deserialize,
        },
        uri,
    };
    use sqlx::postgres::PgConnectOptions;

    /// Start up a new configmonkey app that uses the database pointed by the pg connect options
    pub async fn async_client_from_pg_connect_options(
        pg_connect_options: PgConnectOptions,
    ) -> Client {
        let db_url = format!(
            "postgres://postgres:configmonkey@localhost:5432/{}",
            pg_connect_options.get_database().unwrap()
        );

        let db_config: Map<_, Value> = map! {
            "url" => db_url.into(),
        };

        let figment = rocket::Config::figment()
            .merge(("databases", map!["postgres_configmonkey" => db_config]));

        let client = Client::tracked(rocket_from_config(figment))
            .await
            .expect("valid rocket instance");

        return client;
    }

    /// Create a new domain
    pub async fn h_create_domain<'a>(client: &'a Client, domain_slug: &str) -> LocalResponse<'a> {
        client
            .post(uri!(create_domain))
            .header(ContentType::JSON)
            .body(json!({"slug": domain_slug}).to_string())
            .dispatch()
            .await
    }

    /// Get all available domains
    pub async fn h_get_domains<'a>(
        client: &'a Client,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> LocalResponse<'a> {
        client
            .get(uri!(get_domains(limit, offset)))
            .dispatch()
            .await
    }

    /// Delete domain
    pub async fn h_delete_domain<'a>(client: &'a Client, domain_slug: &str) -> LocalResponse<'a> {
        client
            .delete(uri!(delete_domain(domain_slug)))
            .dispatch()
            .await
    }

    /// Create config
    pub async fn h_create_config<'a>(
        client: &'a Client,
        domain_slug: &str,
        key: &str,
    ) -> LocalResponse<'a> {
        client
            .post(uri!(create_config(domain_slug)))
            .header(ContentType::JSON)
            .body(format!(r#"{{"key": "{}"}}"#, key))
            .dispatch()
            .await
    }

    /// Get all available configs on a specified domain
    pub async fn h_get_configs<'a>(
        client: &'a Client,
        domain_slug: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> LocalResponse<'a> {
        client
            .get(uri!(get_configs(domain_slug, limit, offset)))
            .dispatch()
            .await
    }

    /// Get a specific config
    pub async fn h_get_config<'a>(
        client: &'a Client,
        domain_slug: &str,
        key: &str,
    ) -> LocalResponse<'a> {
        client
            .get(uri!(get_config(domain_slug, key)))
            .dispatch()
            .await
    }

    /// Delete a specific config
    pub async fn h_delete_config<'a>(
        client: &'a Client,
        domain_slug: &str,
        key: &str,
    ) -> LocalResponse<'a> {
        client
            .delete(uri!(delete_config(domain_slug, key)))
            .dispatch()
            .await
    }

    /// Create a config version
    pub async fn h_create_version<'a>(
        client: &'a Client,
        domain_slug: &str,
        key: &str,
        value: rocket::serde::json::Value,
    ) -> LocalResponse<'a> {
        client
            .post(uri!(create_version(domain_slug, key)))
            .header(ContentType::JSON)
            .body(format!(r#"{{"value": {}}}"#, value.to_string()))
            .dispatch()
            .await
    }

    /// Get the list of config versions
    pub async fn h_get_versions<'a>(
        client: &'a Client,
        domain_slug: &str,
        key: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> LocalResponse<'a> {
        client
            .get(uri!(get_versions(domain_slug, key, limit, offset)))
            .dispatch()
            .await
    }

    /// Validate and extract http response body
    pub async fn h_parse_response<'a>(response: LocalResponse<'a>) -> String {
        response.into_string().await.expect("Valid Response Body")
    }

    /// Validate and parse a string into a DTO
    pub fn h_parse_dto<'a, T: Deserialize<'a>>(response_body: &'a str) -> T {
        from_str(response_body).expect("Valid DTO")
    }

    /// Validate pagination
    pub fn h_validate_pagination(
        pagination: PaginationDto,
        expected_count: i32,
        expected_limit: i32,
        expected_offset: i32,
        expected_next: Option<String>,
        expected_prev: Option<String>,
    ) {
        assert_eq!(pagination.count, expected_count);
        assert_eq!(pagination.limit, expected_limit);
        assert_eq!(pagination.offset, expected_offset);
        assert_eq!(pagination.next, expected_next);
        assert_eq!(pagination.prev, expected_prev);
    }
}
