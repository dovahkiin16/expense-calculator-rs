use actix_web::{Responder, HttpResponse, HttpRequest};
use crate::database::user as user_db;

pub async fn find_all(_req: HttpRequest) -> impl Responder {
    let users = user_db::find_all();
    let body = serde_json::to_string(&users).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}