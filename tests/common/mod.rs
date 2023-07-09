#[cfg(test)]
pub mod helpers {
    use configmonkey::{
        app::rocket_from_config,
        routes::v1::apps_routes::{
            rocket_uri_macro_create_app, rocket_uri_macro_get_apps, GetAppsDto,
        },
        routes::v1::envs_routes::rocket_uri_macro_create_env,
    };
    use rocket::{
        figment::{
            map,
            value::{Map, Value},
        },
        http::{ContentType, Status},
        local::asynchronous::{Client, LocalResponse},
        serde::json::serde_json::{from_str, json},
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

    /// Request to create a new app
    pub async fn h_create_app<'a>(
        client: &'a Client,
        app_slug: &str,
        app_name: &str,
    ) -> LocalResponse<'a> {
        client
            .post(uri!(create_app))
            .header(ContentType::JSON)
            .body(json!({"slug": app_slug, "name": app_name }).to_string())
            .dispatch()
            .await
    }

    // Request to get all available apps
    pub async fn h_get_apps<'a>(
        client: &'a Client,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> GetAppsDto {
        let response = client.get(uri!(get_apps(limit, offset))).dispatch().await;

        // assert response
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));

        let response_body = response.into_string().await.expect("Response Body");
        let get_apps_dto: GetAppsDto =
            from_str(&response_body.as_str()).expect("Valid Get Apps Response");
        get_apps_dto
    }

    // Envs

    /// Request to create a new env
    pub async fn h_create_env<'a>(
        client: &'a Client,
        app_slug: &str,
        env_slug: &str,
        env_name: &str,
    ) -> LocalResponse<'a> {
        client
            .post(uri!(create_env(app_slug)))
            .header(ContentType::JSON)
            .body(json!({"slug": env_slug, "name": env_name }).to_string())
            .dispatch()
            .await
    }
}
