use actix_web::web;

use crate::handlers::{
    auth_handler::{login, register},
    edition_group_handler::add_edition_group,
    invitation_handler::send_invitation,
    master_group_handler::add_master_group,
    title_group_handler::add_title_group,
    torrent_handler::upload_torrent,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            // these routes should be protected
            // they are protected as soon as we access the user struct in the handler
            .route("/invitation", web::post().to(send_invitation))
            // .route("/torrent", web::post().to(upload_torrent))
            .route("/master-group", web::post().to(add_master_group))
            .route("/title-group", web::post().to(add_title_group))
            .route("/edition-group", web::post().to(add_edition_group))
            .route("/torrent", web::post().to(upload_torrent)),
    );
}
