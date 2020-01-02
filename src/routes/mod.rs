use actix_web::web;

mod expenses;
mod users;

/// configures all API routes
pub fn config(cfg: &mut web::ServiceConfig) {
    // expenses route configuration
    cfg.service(web::scope("/users/{user_id}/expenses").configure(expenses::config));
    // users route configuration
    cfg.service( web::scope("/users").configure(users::config));

}