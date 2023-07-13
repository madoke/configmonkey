use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PaginationDto {
    pub count: i32,
    pub offset: i32,
    pub limit: i32,
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorMessageDto {
    pub code: String,
    pub message: String,
}
