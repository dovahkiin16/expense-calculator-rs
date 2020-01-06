use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::{error, HttpResponse, Responder};
use failure::Fail;
use serde::Serialize;

pub mod expenses;
pub mod users;

/// For sending Ok responses
fn send_ok(data: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(data)
}

/// used for sending error messages as a serializable
#[derive(Debug, Serialize)]
struct ErrorMessage {
    message: String,
}

/// used for sending user error messages
#[derive(Debug, Fail)]
pub enum UserError {
    #[fail(display = "Validation Error on field: {}", field)]
    ValidationError { field: String },
    #[fail(display = "Username and password didn't match")]
    UsernameAndPasswordNoMatchError,
    #[fail(display = "Username not found")]
    UsernameNotFoundError,
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::UsernameAndPasswordNoMatchError => StatusCode::BAD_REQUEST,
            UserError::UsernameNotFoundError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&ErrorMessage {
            message: self.to_string(),
        }).unwrap();

        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .body(body)
    }
}
