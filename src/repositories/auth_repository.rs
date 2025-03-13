use crate::models::user::{Register, User};
use actix_web::web;
use sqlx::PgPool;
use sqlx::postgres::{PgQueryResult, PgRow};
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
                _ => "Unknown error".to_string(),
            };
            Err(format!("Failed to create user: {}", error_message).into())
        }
    }
}
