use crate::controllers::expenses as expenses_controller;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(expenses_controller::find_all));
    cfg.route("", web::post().to(expenses_controller::add_one));
    cfg.route(
        "/sum",
        web::get().to(expenses_controller::get_total_expense),
    );
}
