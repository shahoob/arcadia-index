use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    Arcadia, Result,
    models::{
        title_group::{
            TitleGroup, TitleGroupAndAssociatedData, TitleGroupInfoLite, UserCreatedTitleGroup,
        },
        user::User,
    },
    repositories::title_group_repository::{
        create_title_group, find_title_group, find_title_group_info_lite,
    },
};

#[utoipa::path(
    post,
    path = "/api/title-group",
    responses(
        (status = 200, description = "Successfully created the title_group", body=TitleGroup),
    )
)]
pub async fn add_title_group(
    form: web::Json<UserCreatedTitleGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = create_title_group(&arc.pool, &form, &current_user).await?;

    Ok(HttpResponse::Created().json(title_group))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTitleGroupQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/title-group",
    params(GetTitleGroupQuery),
    responses(
        (status = 200, description = "Successfully got the title_group", body=TitleGroupAndAssociatedData),
    )
)]
pub async fn get_title_group(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupQuery>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = find_title_group(&arc.pool, query.id, &current_user).await?;

    Ok(HttpResponse::Ok().json(title_group))
}

pub type GetTitleGroupInfoLiteQuery = GetTitleGroupQuery;

#[utoipa::path(
    get,
    path = "/api/title-group/lite",
    params(GetTitleGroupInfoLiteQuery),
    responses(
        (status = 200, description = "Successfully got the title_group (lite info)", body=TitleGroupInfoLite),
    )
)]
pub async fn get_title_group_info_lite(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupInfoLiteQuery>,
) -> Result<HttpResponse> {
    let title_group = find_title_group_info_lite(&arc.pool, query.id).await?;

    Ok(HttpResponse::Ok().json(title_group))
}
