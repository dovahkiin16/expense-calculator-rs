use crate::controllers::users as controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(controller::find_all));
}
