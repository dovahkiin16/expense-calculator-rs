use actix_web::web;

mod expenses;
mod users;

/// configures all API routes
pub fn config(cfg: &mut web::ServiceConfig) {
    // users route configuration
    cfg.service( web::scope("/users").configure(users::config));
    // expenses route configuration
    cfg.service(web::scope("/expenses").configure(expenses::config));
}