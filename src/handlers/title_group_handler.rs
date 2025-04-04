use actix_web::{HttpResponse, web};
use serde::Deserialize;

use crate::{
    Arcadia, Result,
    models::{title_group::UserCreatedTitleGroup, user::User},
    repositories::title_group_repository::{
        create_title_group, find_lite_title_group_info, find_title_group,
    },
};

pub async fn add_title_group(
    form: web::Json<UserCreatedTitleGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = create_title_group(&arc.pool, &form, &current_user).await?;

    Ok(HttpResponse::Created().json(title_group))
}

#[derive(Debug, Deserialize)]
pub struct GetTitleGroupQuery {
    id: i64,
}

pub async fn get_title_group(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupQuery>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = find_title_group(&arc.pool, query.id, &current_user).await?;

    Ok(HttpResponse::Ok().json(title_group))
}

pub type GetLiteTitleGroupInfoQuery = GetTitleGroupQuery;

pub async fn get_lite_title_group_info(
    arc: web::Data<Arcadia>,
    query: web::Query<GetLiteTitleGroupInfoQuery>,
) -> Result<HttpResponse> {
    let title_group = find_lite_title_group_info(&arc.pool, query.id).await?;

    Ok(HttpResponse::Ok().json(title_group))
}
