use actix_http::Request;
use actix_web::{
    App, Error,
    dev::{Service, ServiceResponse},
    test, web,
};
use arcadia_index::{Arcadia, OpenSignups};
use reqwest::Url;
use sqlx::PgPool;
use std::path::PathBuf;

pub async fn create_test_app(
    pool: PgPool,
    open_signups: OpenSignups,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let arc = Arcadia {
        pool,
        open_signups,
        dottorrent_files_path: PathBuf::new(),
        frontend_url: Url::parse("http://testurl").unwrap(),
    };

    // TODO: CORS?
    test::init_service(
        App::new()
            .app_data(web::Data::new(arc))
            .configure(arcadia_index::routes::init),
    )
    .await
}
