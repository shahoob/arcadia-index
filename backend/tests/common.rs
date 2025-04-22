use actix_http::Request;
use actix_web::{
    App, Error,
    body::MessageBody,
    dev::{Service, ServiceResponse},
    test, web,
};
use arcadia_backend::{Arcadia, OpenSignups};
use reqwest::Url;
use serde::de::DeserializeOwned;
use sqlx::PgPool;

pub async fn create_test_app(
    pool: PgPool,
    open_signups: OpenSignups,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let arc = Arcadia {
        pool,
        open_signups,
        tracker_name: String::from("Arcadia Test"),
        frontend_url: Url::parse("http://testurl").unwrap(),
        tracker_url: Url::parse("http://testurl").unwrap(),
    };

    // TODO: CORS?
    test::init_service(
        App::new()
            .app_data(web::Data::new(arc))
            .configure(arcadia_backend::routes::init),
    )
    .await
}

#[allow(dead_code)]
pub async fn read_body_bencode<T: DeserializeOwned, B: MessageBody>(
    resp: ServiceResponse<B>,
) -> Result<T, serde_bencode::Error> {
    let body = test::read_body(resp).await;
    serde_bencode::from_bytes(&body)
}
