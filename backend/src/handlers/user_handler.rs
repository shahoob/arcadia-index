use crate::{
    Arcadia, Error, Result,
    models::user::{Profile, PublicProfile, User, UserCreatedUserWarning, UserWarning},
    repositories::{
        peer_repository,
        user_repository::{create_user_warning, find_user_profile, find_user_warnings},
    },
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use serde_json::json;
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
        (status = 200, description = "Successfully got the user's profile", body=PublicProfile),
    )
)]
pub async fn get_user(
    arc: web::Data<Arcadia>,
    query: web::Query<GetUserQuery>,
) -> Result<HttpResponse> {
    let user = find_user_profile(&arc.pool, &query.id).await?;

    Ok(HttpResponse::Created().json(json!({"user":user})))
}

#[utoipa::path(
    get,
    path = "/api/me",
    responses(
        (status = 200, description = "Successfully got the user's profile", body=Profile),
    )
)]
pub async fn get_me(mut current_user: User, arc: web::Data<Arcadia>) -> HttpResponse {
    current_user.password_hash = String::from("");
    let peers = peer_repository::get_user_peers(&arc.pool, current_user.id).await;
    let user_warnings = find_user_warnings(&arc.pool, current_user.id).await;
    HttpResponse::Ok()
        .json(json!({"user": current_user, "peers":peers, "user_warnings": user_warnings}))
}

#[utoipa::path(
    post,
    path = "/api/user/warn",
    responses(
        (status = 200, description = "Successfully warned the user", body=UserWarning),
    )
)]
pub async fn warn_user(
    form: web::Json<UserCreatedUserWarning>,
    current_user: User,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    if current_user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }
    let user_warning = create_user_warning(&arc.pool, current_user.id, &form).await?;

    Ok(HttpResponse::Created().json(user_warning))
}
