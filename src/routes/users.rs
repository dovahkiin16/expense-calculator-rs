use actix_web::web;
use crate::controllers::users as controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(controller::find_all));
}