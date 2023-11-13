use chrono::{DateTime, Utc};
pub struct Domain {
    pub id: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}
