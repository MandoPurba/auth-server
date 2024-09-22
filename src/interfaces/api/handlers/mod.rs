use actix_web::web;
use auth_handlers::register_user;

pub mod auth_handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
}
