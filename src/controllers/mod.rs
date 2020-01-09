use actix_web::{HttpResponse, Responder};

pub mod errors;
pub mod expenses;
pub mod users;

/// For sending Ok responses
fn send_ok(data: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(data)
}
