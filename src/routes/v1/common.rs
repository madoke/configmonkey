use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorMessageDto<'r> {
    pub code: &'r str,
    pub message: &'r str,
}
