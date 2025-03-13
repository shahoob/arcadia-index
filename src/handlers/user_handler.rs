use std::{net::IpAddr, str::FromStr, time::Duration};

use actix_web::{HttpRequest, HttpResponse, web};
use chrono::Utc;

use crate::models::user::{Register, User};

// pub async fn get_user(user_id: web::Data<u32>) -> HttpResponse {
//     // For now, we'll just mock a user response.
//     let mock_user = User {
//         id: **user_id,
//         username: String::from("mock_user"),
//         email: String::from("mock_user@example.com"),
//         created_from_ip: IpAddr::from_str("127.0.0.1").unwrap(),
//     };

//     HttpResponse::Ok().json(mock_user)
// }

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is up and running!")
}
