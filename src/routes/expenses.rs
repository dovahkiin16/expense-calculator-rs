use actix_web::web;
use crate::controllers::expenses as expenses_controller;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(expenses_controller::find_all));
}
