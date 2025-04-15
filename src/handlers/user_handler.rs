use crate::{
    Arcadia, Result,
    models::user::{PublicUser, User},
    repositories::user_repository::find_user_by_id,
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetUserQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/user",
    params(GetUserQuery),
    responses(
        (status = 200, description = "Successfully got the user", body=PublicUser),
    )
)]
pub async fn get_user(
    arc: web::Data<Arcadia>,
    query: web::Query<GetUserQuery>,
) -> Result<HttpResponse> {
    let user = find_user_by_id(&arc.pool, &query.id).await?;

    Ok(HttpResponse::Created().json(user))
}

#[utoipa::path(
    get,
    path = "/api/me",
    responses(
        (status = 200, description = "Successfully got the user", body=User),
    )
)]
pub async fn get_me(mut current_user: User) -> HttpResponse {
    current_user.password_hash = String::from("");
    HttpResponse::Ok().json(current_user)
}
