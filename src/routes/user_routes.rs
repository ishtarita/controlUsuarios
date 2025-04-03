use actix_web::web;
use crate::handlers::user_handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(user_handler::get_users);
}
