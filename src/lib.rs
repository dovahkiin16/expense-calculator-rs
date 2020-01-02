#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod database;
pub mod models;
pub mod schema;

mod controllers;
pub mod routes;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
