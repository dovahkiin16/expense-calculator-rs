use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::{error, HttpResponse};

use failure::Fail;

/// used for sending user error messages
#[derive(Debug, Fail)]
pub enum UserError {
    #[fail(display = "Username and password didn't match")]
    UsernameAndPasswordNoMatchError,
    #[fail(display = "Username not found")]
    UsernameNotFoundError,
}

impl error::ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::UsernameAndPasswordNoMatchError => StatusCode::BAD_REQUEST,
            UserError::UsernameNotFoundError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&super::ErrorMessage {
            message: self.to_string(),
        })
        .unwrap();

        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .body(body)
    }
}
