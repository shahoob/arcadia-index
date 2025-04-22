use actix_web::{
    http::{StatusCode, header::HeaderValue},
    test,
};
use serde::Deserialize;
use sqlx::PgPool;

mod common;

use arcadia_backend::OpenSignups;

#[sqlx::test(fixtures("with_test_user", "with_test_torrent"))]
async fn test_valid_torrent(pool: PgPool) {
    let service = common::create_test_app(pool, OpenSignups::Enabled).await;

    // Login first
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/login")
        .set_json(serde_json::json!({
            "username": "test_user",
            "password": "test_password",
            "remember_me": true,
        }))
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    #[derive(PartialEq, Deserialize)]
    struct UserResponse {
        username: String,
        id: i64,
        avatar: Option<String>,
    }

    #[derive(PartialEq, Deserialize)]
    struct LoginResponse {
        token: String,
        user: UserResponse,
    }

    let user = test::read_body_json::<LoginResponse, _>(resp).await;

    assert!(!user.token.is_empty());
    assert_eq!(user.user.username, "test_user");
    assert!(user.user.avatar.is_none());

    let req = test::TestRequest::get()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .insert_header(("Authorization", format!("Bearer {}", user.token)))
        .uri("/api/torrent?id=1")
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    // A minimum set of definitions for assertions.
    #[derive(Debug, Deserialize)]
    struct Info {
        private: isize,
    }

    #[derive(Debug, Deserialize)]
    struct MetaInfo {
        info: Info,
        announce: String,
    }

    let metainfo = common::read_body_bencode::<MetaInfo, _>(resp)
        .await
        .expect("could not deserialize metainfo");

    assert_eq!(
        metainfo.info.private, 1,
        "expected downloaded torrent to be private"
    );

    let test_user_passkey = "d2037c66dd3e13044e0d2f9b891c3837";
    assert!(
        metainfo.announce.contains(test_user_passkey),
        "expected announce url to contain test_user passkey"
    );
}
