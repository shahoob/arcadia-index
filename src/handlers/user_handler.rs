use crate::{Arcadia, Result, models::user::User, repositories::user_repository::find_user_by_id};
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetUserQuery {
    id: i64,
}

pub async fn get_user(
    arc: web::Data<Arcadia>,
    query: web::Query<GetUserQuery>,
) -> Result<HttpResponse> {
    let user = find_user_by_id(&arc.pool, &query.id).await?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn get_me(mut current_user: User) -> HttpResponse {
    current_user.password_hash = String::from("");
    HttpResponse::Ok().json(current_user)
}
