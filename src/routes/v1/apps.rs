use crate::{db::db::ConfigMonkeyDb, services::apps_svc};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::Connection;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAppInput<'r> {
    slug: &'r str,
    name: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AppDto {
    pub id: String,
    pub slug: String,
    pub name: String,
}

#[get("/v1/apps")]
pub async fn get_apps(db: Connection<ConfigMonkeyDb>) -> Json<Vec<AppDto>> {
    let result = apps_svc::get_apps(db).await;
    let mut appdtos = vec![];

    return match result {
        Ok(apps) => {
            for app in apps {
                appdtos.push(AppDto {
                    name: app.name,
                    slug: app.slug,
                    id: app.id,
                });
            }
            Json(appdtos)
        }
        Err(_e) => panic!("Panix !"),
    };
}

#[post("/v1/apps", data = "<input>")]
pub async fn create_app(
    db: Connection<ConfigMonkeyDb>,
    input: Json<CreateAppInput<'_>>,
) -> Json<AppDto> {
    let result = apps_svc::create_app(db, String::from(input.slug), String::from(input.name)).await;

    return match result {
        Ok(app) => Json(AppDto {
            name: app.name,
            slug: app.slug,
            id: app.id,
        }),
        Err(_e) => panic!("Panix"),
    };
}
