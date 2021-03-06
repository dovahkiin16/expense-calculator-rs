#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;
extern crate env_logger;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod database;
pub mod models;
pub mod schema;

pub mod controllers;
pub mod routes;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    env_logger::init();
    let port = env::var("PORT").unwrap_or(String::from("8001"));
    let ip_bind = "0.0.0.0";

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .configure(routes::config)
    })
    .bind(format!("{}:{}", ip_bind, port))?
    .run()
    .await
}
