use crate::{
    Arcadia,
    models::{
        invitation::Invitation,
        user::{Claims, Login, Register},
    },
    repositories::{
        auth_repository::{create_user, find_user_with_password},
        invitation_repository::does_unexpired_invitation_exist,
    },
};
use actix_web::{HttpRequest, HttpResponse, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::Duration;
use chrono::prelude::Utc;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use sqlx::types::ipnetwork::IpNetwork;
use std::env;

#[derive(Debug, Deserialize)]
pub struct RegisterQuery {
    invitation_key: Option<String>,
}

// #[utoipa::path(
//     get,
//     path = "/api/register",
//     responses(
//         (status = 200, description = "Successfully registered the user", body = Register),
//     )
// )]
pub async fn register(
    new_user: web::Json<Register>,
    arc: web::Data<Arcadia>,
    req: HttpRequest,
    query: web::Query<RegisterQuery>,
) -> HttpResponse {
    let invitation: Invitation;
    if !arc.is_open_signups() {
        let Some(invitation_key) = &query.invitation_key else {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "invitation key not found in query"
            }));
        };
        match does_unexpired_invitation_exist(&arc.pool, &invitation_key).await {
            Ok(invitation_found) => {
                if invitation_found.receiver_id.is_some() {
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "invitation already used"
                    }));
                }
                invitation = invitation_found
            }
            Err(err) => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": err.to_string()
                }));
            }
        }
    } else {
        invitation = Invitation::default();
    }

    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpNetwork>().ok())
        .unwrap();

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(new_user.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    match create_user(
        &arc.pool,
        &new_user,
        client_ip,
        &password_hash,
        &invitation,
        &arc.is_open_signups(),
    )
    .await
    {
        Ok(user) => HttpResponse::Created().json(serde_json::json!(user)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub async fn login(arc: web::Data<Arcadia>, user_login: web::Json<Login>) -> HttpResponse {
    match find_user_with_password(&arc.pool, &user_login).await {
        Ok(user) => {
            let mut expiration_date = Utc::now();
            if !user_login.remember_me {
                expiration_date = expiration_date + Duration::hours(1);
            } else {
                expiration_date = expiration_date + Duration::days(30);
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
                "token": token,
                "user": {"username": user.username, "id": user.id, "avatar": user.avatar}
            })))
        }
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
