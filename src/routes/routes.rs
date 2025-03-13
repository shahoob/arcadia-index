use actix_web::web;

use crate::handlers::{
    auth_handler::{login, register},
    invitation_handler::send_invitation,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            // these routes should be protected
            // they are protected as soon as we access the user struct in the handler
            .route("/invitation", web::post().to(send_invitation)),
    );
}
