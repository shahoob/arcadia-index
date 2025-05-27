use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    Arcadia, Result,
    models::{
        title_group::{
            TitleGroup, TitleGroupAndAssociatedData, TitleGroupLite, UserCreatedTitleGroup,
        },
        user::User,
    },
    repositories::{
        artist_repository::create_artists_affiliation,
        title_group_repository::{
            create_title_group, find_title_group, find_title_group_info_lite,
        },
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
    mut form: web::Json<UserCreatedTitleGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let created_title_group = create_title_group(&arc.pool, &form, &current_user).await?;

    for artist in &mut form.affiliated_artists {
        artist.title_group_id = created_title_group.id
    }

    let _ =
        create_artists_affiliation(&arc.pool, &form.affiliated_artists, current_user.id).await?;

    Ok(HttpResponse::Created().json(created_title_group))
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

pub type GetTitleGroupLiteQuery = GetTitleGroupQuery;

#[utoipa::path(
    get,
    path = "/api/title-group/lite",
    params(GetTitleGroupLiteQuery),
    responses(
        (status = 200, description = "Successfully got the title_group (lite info)", body=TitleGroupLite),
    )
)]
pub async fn get_title_group_info_lite(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupLiteQuery>,
) -> Result<HttpResponse> {
    let title_group = find_title_group_info_lite(&arc.pool, query.id).await?;

    Ok(HttpResponse::Ok().json(title_group))
}
