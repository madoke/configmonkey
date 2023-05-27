use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct App {
    pub id: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
