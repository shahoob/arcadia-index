use std::{net::IpAddr, str::FromStr};

use actix_web::{HttpResponse, web};

use crate::models::user::{CreateUser, User};

pub async fn create_user(user: web::Json<CreateUser>) -> HttpResponse {
    let new_user = User {
        id: 0, //TODO
        username: user.username.clone(),
        email: user.email.clone(),
        created_from_ip: IpAddr::from_str("127.0.0.1").unwrap(),
    };

    HttpResponse::Created().json(new_user)
}

pub async fn get_user(user_id: web::Data<u32>) -> HttpResponse {
    // For now, we'll just mock a user response.
    let mock_user = User {
        id: **user_id,
        username: String::from("mock_user"),
        email: String::from("mock_user@example.com"),
        created_from_ip: IpAddr::from_str("127.0.0.1").unwrap(),
    };

    HttpResponse::Ok().json(mock_user)
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is up and running!")
}
