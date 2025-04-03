use actix_web::{HttpResponse, web};
use serde::Deserialize;

use crate::{
    Arcadia,
    models::{title_group::UserCreatedTitleGroup, user::User},
    repositories::title_group_repository::{
        create_title_group, find_lite_title_group_info, find_title_group,
    },
};

pub async fn add_title_group(
    form: web::Json<UserCreatedTitleGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> HttpResponse {
    match create_title_group(&arc.pool, &form, &current_user).await {
        Ok(title_group) => HttpResponse::Created().json(serde_json::json!(title_group)),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

#[derive(Debug, Deserialize)]
pub struct GetTitleGroupQuery {
    id: i64,
}

pub async fn get_title_group(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupQuery>,
    current_user: User,
) -> HttpResponse {
    match find_title_group(&arc.pool, query.id, &current_user).await {
        Ok(title_group) => HttpResponse::Ok().json(title_group),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}

pub type GetLiteTitleGroupInfoQuery = GetTitleGroupQuery;

pub async fn get_lite_title_group_info(
    arc: web::Data<Arcadia>,
    query: web::Query<GetLiteTitleGroupInfoQuery>,
) -> HttpResponse {
    match find_lite_title_group_info(&arc.pool, query.id).await {
        Ok(title_group) => HttpResponse::Ok().json(title_group),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": err.to_string()
        })),
    }
}
