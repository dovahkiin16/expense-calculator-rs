use crate::database::user as user_db;
use crate::models::NewUser;
use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn find_all(_req: HttpRequest) -> impl Responder {
    let users = user_db::find_all();
    let body = serde_json::to_string(&users).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

pub async fn add_one(item: web::Json<NewUser>) -> impl Responder {
    let pwd = bcrypt::hash(item.password.clone(), 10).unwrap();

    let insertable_user = NewUser {
        name: item.name.clone(),
        username: item.username.clone(),
        password: pwd,
    };

    let new_user = user_db::add_one(insertable_user).unwrap();
    let body = serde_json::to_string(&new_user).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}
