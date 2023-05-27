use chrono::Utc;

use crate::models::env::Env;

pub fn get_envs() -> Vec<Env> {
    let mut result: Vec<Env> = Vec::new();

    result.push(Env {
        id: String::from("123"),
        slug: String::from("prod"),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });
    result.push(Env {
        id: String::from("789"),
        slug: String::from("dev"),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });

    return result;
}
