use actix_web::{Responder, HttpResponse, web};
use serde::Deserialize;

use crate::database::expense as expense_db;

#[derive(Deserialize)]
pub struct FindAllPath {
    pub user_id: i32
}

pub async fn find_all(info: web::Path<FindAllPath>) -> impl Responder {
    let user_expenses = expense_db::find_all(info.user_id);
    let body = serde_json::to_string(&user_expenses).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}