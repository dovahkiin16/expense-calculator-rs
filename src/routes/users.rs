use crate::controllers::users as users_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(users_controller::find_all));
    cfg.route("", web::post().to(users_controller::add_one));
    cfg.route("/login", web::post().to(users_controller::login));
}
