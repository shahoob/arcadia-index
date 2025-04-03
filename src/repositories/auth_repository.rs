use crate::models::{
    invitation::Invitation,
    user::{Claims, Login, Register, User},
};
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use futures::future::BoxFuture;
use jsonwebtoken::{DecodingKey, Validation, decode};
use sqlx::{PgPool, types::ipnetwork::IpNetwork};
use std::{env, error::Error};

pub async fn create_user(
    pool: &PgPool,
    user: &Register,
    from_ip: IpNetwork,
    password_hash: &str,
    invitation: &Invitation,
    open_signups: &bool,
) -> Result<User, Box<dyn Error>> {
    let registered_user = sqlx::query_as!(
        User,
        r#"
            INSERT INTO users (username, email, password_hash, registered_from_ip)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
        &user.username,
        &user.email,
        password_hash,
        from_ip
    )
    .fetch_one(pool)
    .await;

    match registered_user {
        Ok(_) => {
            let registered_user = registered_user.unwrap();
            if !*open_signups {
                // TODO: check this properly
                let _ = sqlx::query!(
                    r#"
                    UPDATE invitations SET receiver_id = $1 WHERE id = $2;
                    "#,
                    registered_user.id,
                    invitation.id
                )
                .execute(pool)
                .await;
            }

            Ok(registered_user)
        }
        Err(e) => {
            let error_message = match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("Failed to create user: {}", error_message).into())
        }
    }
}

pub async fn find_user_with_password(pool: &PgPool, user: &Login) -> Result<User, Box<dyn Error>> {
    let result = sqlx::query_as!(
        User,
        r#"
            SELECT * FROM users
            WHERE username = $1
        "#,
        user.username
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(_) => {
            let result = result.unwrap();
            let parsed_hash = PasswordHash::new(&result.password_hash);
            if Argon2::default()
                .verify_password(user.password.as_bytes(), &parsed_hash.unwrap())
                .is_ok()
            {
                Ok(result)
            } else {
                Err("wrong username or password".into())
            }
        }
        Err(e) => {
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("User not found").into())
        }
    }
}

// user provider, which also acts as the auth system
impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<PgPool>>().unwrap().clone();
        let auth_header = req.headers().get("Authorization").cloned();

        Box::pin(async move {
            if let Some(auth_value) = auth_header {
                if let Ok(auth_str) = auth_value.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        let decoding_key =
                            DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());
                        let validation = Validation::default();

                        if let Ok(token_data) = decode::<Claims>(token, &decoding_key, &validation)
                        {
                            let user_id = token_data.claims.sub;

                            let user = sqlx::query_as!(
                                User,
                                r#"
                                    UPDATE users
                                    SET last_seen = NOW()
                                    WHERE id = $1
                                    RETURNING *
                                "#,
                                user_id
                            )
                            .fetch_one(pool.get_ref())
                            .await
                            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()));

                            return user.map_err(|_| {
                                actix_web::error::ErrorUnauthorized("User not found")
                            });
                        }
                    }
                }
            }
            Err(actix_web::error::ErrorUnauthorized("authentication error"))
        })
    }
}
