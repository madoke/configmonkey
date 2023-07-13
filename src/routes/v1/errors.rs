use std::io::Cursor;

use rocket::{
    catch,
    http::{ContentType, Status},
    response::Responder,
    serde::json::{to_string, Json},
    Request, Response,
};

use super::dtos::ErrorMessageDto;

pub struct RoutesError(pub Status, pub &'static str, pub &'static str);

impl<'a> Responder<'a, 'static> for RoutesError {
    /// Generic responder that builds error responses
    fn respond_to(self, _: &'a Request<'_>) -> rocket::response::Result<'static> {
        let RoutesError(http_status, error_code, error_message) = self;

        let response_body = to_string(&ErrorMessageDto {
            code: error_code.to_string(),
            message: error_message.to_string(),
        })
        .unwrap();

        Response::build()
            .header(ContentType::JSON)
            .status(http_status)
            .sized_body(response_body.len(), Cursor::new(response_body))
            .ok()
    }
}

#[catch(404)]
pub fn not_found() -> Json<ErrorMessageDto> {
    return Json(ErrorMessageDto {
        code: "resource_not_found".to_string(),
        message: "Resource not found".to_string(),
    });
}

#[catch(default)]
pub fn default_catcher() -> Json<ErrorMessageDto> {
    return Json(ErrorMessageDto {
        code: "unknown_error".to_string(),
        message: "Unknown Error".to_string(),
    });
}
