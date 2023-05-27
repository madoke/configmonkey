use chrono::Utc;
use uuid::Uuid;

use crate::models::app::App;

static mut APPS: Vec<App> = Vec::new();

pub fn get_apps() -> Vec<App> {
    return unsafe { APPS.clone() };
}

pub fn create_app(slug: String) {
    let app = App {
        id: Uuid::new_v4().to_string(),
        slug: slug,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    unsafe { APPS.push(app) };
}
