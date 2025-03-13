use actix_web::{HttpRequest, HttpResponse, web};
use chrono::Duration;
use chrono::prelude::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;
use std::{env, net::IpAddr};

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::{
    models::user::{Claims, Login, Register},
    repositories::auth_repository::{create_user, find_user_with_password},
};

pub async fn register(
    new_user: web::Json<Register>,
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {
    let client_ip: IpAddr = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|ip| ip.parse().ok())
        .unwrap();

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
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub async fn login(pool: web::Data<PgPool>, user_login: web::Json<Login>) -> HttpResponse {
    match find_user_with_password(&pool, &user_login).await {
        Ok(user) => {
            let mut expiration_date = Utc::now();
            if !user_login.remember_me {
                expiration_date = expiration_date + Duration::hours(1);
            } else {
                expiration_date = expiration_date + Duration::days(365);
            }
            let user_claims = Claims {
                sub: user.id,
                exp: expiration_date.timestamp() as usize,
            };
            let token = encode(
                &Header::default(),
                &user_claims,
                &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
            )
            .unwrap();

            HttpResponse::Ok().json(serde_json::json!(serde_json::json!({
                "token": token
            })))
        }
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
