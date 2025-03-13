use crate::models::user::Register;
use actix_web::web;
use sqlx::PgPool;
use std::error::Error;
use std::net::IpAddr;

pub async fn create_user(
    pool: &web::Data<PgPool>,
    user: &Register,
    from_ip: &IpAddr,
    password_hash: &str,
) -> Result<(), Box<dyn Error>> {
    // let user = user.into_inner();

    let query = r#"
        INSERT INTO users (username, email, password_hash, registered_from_ip) 
        VALUES ($1, $2, $3, $4::inet)
    "#;

    let result = sqlx::query(query)
        .bind(&user.username)
        .bind(&user.email)
        .bind(password_hash)
        .bind(from_ip.to_string())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create user: {:?}", e).into()),
    }
}
