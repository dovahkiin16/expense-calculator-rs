use actix_web::http::{header, StatusCode};
use actix_web::{error, HttpResponse};

use actix_web::dev::HttpResponseBuilder;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum ExpenseError {
    #[fail(display = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for ExpenseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ExpenseError::ValidationError { .. } => StatusCode::BAD_REQUEST,
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
