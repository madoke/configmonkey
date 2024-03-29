use std::io::Cursor;

use rocket::{
    catch,
    http::{ContentType, Status},
    response::Responder,
    serde::json::{to_string, Json},
    Request, Response,
};

use super::dtos::ErrorDto;

pub struct RoutesError(pub Status, pub &'static str, pub &'static str);

impl<'a> Responder<'a, 'static> for RoutesError {
    /// Generic responder that builds error responses
    fn respond_to(self, _: &'a Request<'_>) -> rocket::response::Result<'static> {
        let RoutesError(http_status, error_code, error_message) = self;

        let response_body = to_string(&ErrorDto {
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
pub fn not_found() -> Json<ErrorDto> {
    return Json(ErrorDto {
        code: "not_found".to_string(),
        message: "Resource not found".to_string(),
    });
}

#[catch(400)]
pub fn bad_request() -> Json<ErrorDto> {
    return Json(ErrorDto {
        code: "bad_request".to_string(),
        message: "Unable to parse input parameters".to_string(),
    });
}

#[catch(default)]
pub fn default_catcher() -> Json<ErrorDto> {
    return Json(ErrorDto {
        code: "unknown".to_string(),
        message: "Unknown Error".to_string(),
    });
}
