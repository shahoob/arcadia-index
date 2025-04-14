use crate::{
    Arcadia, Error, Result,
    models::{
        invitation::Invitation,
        user::{Claims, Login, Register, User},
    },
};
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use futures::future::BoxFuture;
use jsonwebtoken::{DecodingKey, Validation, decode};
use rand::Rng;
use sqlx::{PgPool, types::ipnetwork::IpNetwork};
use std::env;

pub async fn create_user(
    pool: &PgPool,
    user: &Register,
    from_ip: IpNetwork,
    password_hash: &str,
    invitation: &Invitation,
    open_signups: &bool,
) -> Result<User> {
    let mut rng = rand::rng();

    let passkey = rng.random::<u128>();

    let passkey_upper = (passkey >> 64) as i64;
    let passkey_lower = passkey as i64;

    let registered_user = sqlx::query_as!(
        User,
        r#"
            INSERT INTO users (username, email, password_hash, registered_from_ip, passkey_upper, passkey_lower)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
        "#,
        &user.username,
        &user.email,
        password_hash,
        from_ip,
        passkey_upper,
        passkey_lower
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateUser)?;

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

pub async fn find_user_with_password(pool: &PgPool, login: &Login) -> Result<User> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT * FROM users
            WHERE username = $1
        "#,
        login.username
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::WrongUsernameOrPassword)?;

    let parsed_hash = PasswordHash::new(&user.password_hash);

    Argon2::default()
        .verify_password(login.password.as_bytes(), &parsed_hash.unwrap())
        .map_err(|_| Error::WrongUsernameOrPassword)?;

    Ok(user)
}

// user provider, which also acts as the auth system
impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = BoxFuture<'static, std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = req.app_data::<web::Data<Arcadia>>().unwrap().pool.clone();
        let auth_header = req.headers().get("Authorization").cloned();

        Box::pin(async move {
            let auth_value =
                auth_header.ok_or(actix_web::error::ErrorUnauthorized("authentication error"))?;

            let auth_str = auth_value
                .to_str()
                .map_err(|_| actix_web::error::ErrorUnauthorized("authentication error"))?;

            let token = auth_str
                .strip_prefix("Bearer ")
                .ok_or(actix_web::error::ErrorUnauthorized("authentication error"))?;

            let decoding_key = DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());

            let validation = Validation::default();

            let token_data = decode::<Claims>(token, &decoding_key, &validation)
                .map_err(|_| actix_web::error::ErrorUnauthorized("authentication error"))?;

            let user = sqlx::query_as!(
                User,
                r#"
                    UPDATE users
                    SET last_seen = NOW()
                    WHERE id = $1
                    RETURNING *
                "#,
                token_data.claims.sub
            )
            .fetch_one(&pool)
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            Ok(user)
        })
    }
}
