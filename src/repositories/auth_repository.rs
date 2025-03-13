use crate::models::user::{Login, Register, User};
use actix_web::web;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;
use std::error::Error;
use std::net::IpAddr;

pub async fn create_user(
    pool: &web::Data<PgPool>,
    user: &Register,
    from_ip: &IpAddr,
    password_hash: &str,
) -> Result<User, Box<dyn Error>> {
    // let user = user.into_inner();

    let query = r#"
        INSERT INTO users (username, email, password_hash, registered_from_ip) 
        VALUES ($1, $2, $3, $4::inet)
        RETURNING *
    "#;

    let result = sqlx::query_as::<_, User>(query)
        .bind(&user.username)
        .bind(&user.email)
        .bind(password_hash)
        .bind(from_ip.to_string())
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(_) => Ok(result.unwrap()),
        Err(e) => {
            let error_message = match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("Failed to create user: {}", error_message).into())
        }
    }
}

pub async fn find_user_with_password(
    pool: &web::Data<PgPool>,
    user: &Login,
) -> Result<User, Box<dyn Error>> {
    let query = r#"
        SELECT * FROM users
        WHERE username = $1
    "#;

    let result = sqlx::query_as::<_, User>(query)
        .bind(&user.username)
        .fetch_one(pool.get_ref())
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
                Err("Wrong password".into())
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
