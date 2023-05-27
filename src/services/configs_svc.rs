use chrono::Utc;

use crate::models::config::Config;

pub fn get_configs() -> Vec<Config> {
    let mut result: Vec<Config> = Vec::new();

    result.push(Config {
        id: String::from("123"),
        slug: String::from("xoxo"),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });
    result.push(Config {
        id: String::from("789"),
        slug: String::from("xoxo"),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    });

    return result;
}
