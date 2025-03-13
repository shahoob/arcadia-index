use actix_web::web;

use crate::handlers::auth_handler::register;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").route("/register", web::post().to(register)), // .route("/users/{id}", web::get().to(get_user))
                                                                         // .route("/health", web::get().to(health_check)),
    );
}
