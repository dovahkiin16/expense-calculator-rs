use actix_web::{Responder, HttpResponse, HttpRequest};
use crate::database::user;

pub async fn find_all(_req: HttpRequest) -> impl Responder {
    let users = user::find_all();
    let body = serde_json::to_string(&users).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}