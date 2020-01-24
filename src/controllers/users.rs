use crate::database::user as user_db;
use crate::models::NewUser;

use actix_web::web;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Used for extracting login
#[derive(Debug, Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

/// get all users
pub async fn find_all(_req: HttpRequest) -> impl Responder {
    let users = user_db::find_all();
    let body = serde_json::to_string(&users).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

/// create new user
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

/// login using username and password
pub async fn login(item: web::Json<UserLogin>) -> Result<impl Responder, super::errors::UserError> {
    let user = user_db::find_one_by_username(&item.username);
    let none = user.is_none();
    // check if no user is found using username
    if none {
        return Err(super::errors::UserError::UsernameNotFoundError);
    }

    let user = user.unwrap();
    // check if password match
    if user.verify_password(&item.password) {
        let user_display = user.to_display_user();
        let body = serde_json::to_string(&user_display).unwrap();

        Ok(super::send_ok(body))
    } else {
        Err(super::errors::UserError::UsernameAndPasswordNoMatchError)
    }
}
