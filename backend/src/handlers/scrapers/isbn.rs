use actix_web::{HttpResponse, web};
use google_books1::{Books, hyper_rustls, hyper_util, yup_oauth2};

use serde::Deserialize;
use utoipa::IntoParams;

use crate::{Arcadia, Result, handlers::scrapers::ExternalDBData};

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetISBNQuery {
    isbn: String,
}

#[utoipa::path(
    post,
    params(GetISBNQuery),
    path = "/api/external_db/isbn",
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn get_isbn_data(
    query: web::Query<GetISBNQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let secret: yup_oauth2::ApplicationSecret = yup_oauth2::ApplicationSecret {
        client_id: "".to_string(),
        client_secret: "".to_string(),
        ..Default::default()
    };
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );
    let hub = Books::new(client, auth);
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub
        .promooffer()
        .accept()
        .volume_id("amet.")
        .serial("takimata")
        .product("amet.")
        .offer_id("duo")
        .model("ipsum")
        .manufacturer("gubergren")
        .device("Lorem")
        .android_id("gubergren")
        .doit()
        .await;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        // "title_group": None,
        // "edition_group": None
    })))
}
