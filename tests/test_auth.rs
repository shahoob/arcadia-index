use actix_http::Request;
use actix_web::{
    App, Error,
    dev::{Service, ServiceResponse},
    http::{
        StatusCode,
        header::{self, ContentType, HeaderValue},
    },
    test, web,
};
use arcadia_index::Arcadia;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

async fn create_test_app(
    pool: PgPool,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    // TODO: CORS?
    test::init_service(
        App::new()
            .app_data(web::Data::new(Arcadia { pool }))
            .configure(arcadia_index::routes::init),
    )
    .await
}

#[derive(PartialEq, Debug, Serialize)]
struct RegisterRequest<'a> {
    username: &'a str,
    password: &'a str,
    password_verify: &'a str,
    email: &'a str,
}

#[derive(PartialEq, Debug, Deserialize)]
struct RegisterResponse {
    username: String,
    email: String,
    registered_from_ip: String,
}

#[sqlx::test]
async fn test_open_registration(pool: PgPool) {
    // TODO: rework to include signup config as app state
    if std::env::var("ARCADIA_OPEN_SIGNUPS").unwrap() != "true" {
        return;
    }

    let service = create_test_app(pool).await;

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/register")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    let user = test::read_body_json::<RegisterResponse, _>(resp).await;

    assert_eq!(
        user,
        RegisterResponse {
            username: "test_user".into(),
            email: "test_email@testdomain.com".into(),
            // TODO: strip unnecessary /32 host postfix
            registered_from_ip: "10.10.4.88/32".into(),
        }
    );
}

#[sqlx::test(fixtures("with_test_user", "with_test_user_invite"))]
async fn test_closed_registration_failures(pool: PgPool) {
    // TODO: rework to include signup config as app state
    if std::env::var("ARCADIA_OPEN_SIGNUPS").unwrap() == "true" {
        return;
    }

    let service = create_test_app(pool).await;

    // No key specified.  Should fail.
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/register")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    // TODO: change to FORBIDDEN
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    // Invalid key specified.  Should fail.
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/register?invitation_key=invalid")
        .set_json(RegisterRequest {
            username: "test_user",
            password: "test_password",
            password_verify: "test_password",
            email: "test_email@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    // TODO: change to FORBIDDEN
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );
}

#[sqlx::test(fixtures("with_test_user", "with_test_user_invite"))]
async fn test_closed_registration_success(pool: PgPool) {
    // TODO: rework to include signup config as app state
    if std::env::var("ARCADIA_OPEN_SIGNUPS").unwrap() == "true" {
        return;
    }

    let service = create_test_app(pool).await;

    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user2",
            password: "test_password2",
            password_verify: "test_password2",
            email: "newuser@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(
        resp.headers().get("Content-Type"),
        Some(&HeaderValue::from_static("application/json"))
    );

    let user = test::read_body_json::<RegisterResponse, _>(resp).await;

    assert_eq!(
        user,
        RegisterResponse {
            username: "test_user2".into(),
            email: "newuser@testdomain.com".into(),
            // TODO: strip unnecessary /32 host postfix
            registered_from_ip: "10.10.4.88/32".into(),
        }
    );

    // Try again with same key.  Should fail.
    let req = test::TestRequest::post()
        .insert_header(("X-Forwarded-For", "10.10.4.88"))
        .uri("/api/register?invitation_key=valid_key")
        .set_json(RegisterRequest {
            username: "test_user3",
            password: "test_password3",
            password_verify: "test_password3",
            email: "newuser2@testdomain.com",
        })
        .to_request();

    let resp = test::call_service(&service, req).await;

    // TODO: change to FORBIDDEN
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[sqlx::test(fixtures("with_test_user"))]
async fn test_login_success(pool: PgPool) {
    let service = create_test_app(pool).await;

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
}
