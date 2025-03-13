use std::net::IpAddr;

use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::{models::user::Register, repositories::auth_repository::create_user};

pub async fn register(
    user_register: web::Json<Register>,
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {
    let client_ip: IpAddr = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|ip| ip.parse().ok())
        .unwrap();

    let new_user = Register {
        ..user_register.into_inner()
    };

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(new_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    match create_user(&pool, &new_user, &client_ip, &password_hash).await {
        Ok(user) => HttpResponse::Created().json(serde_json::json!(user)),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
