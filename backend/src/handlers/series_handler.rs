use crate::{
    Arcadia, Result,
    models::{
        series::{Series, SeriesAndTitleGroupHierarchyLite, UserCreatedSeries},
        user::User,
    },
    repositories::series_repository::{create_series, find_series},
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;
use utoipa::IntoParams;

#[utoipa::path(
    post,
    path = "/api/series",
    responses(
        (status = 200, description = "Successfully created the series", body=Series),
    )
)]
pub async fn add_series(
    serie: web::Json<UserCreatedSeries>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let series = create_series(&arc.pool, &serie, &current_user).await?;

    Ok(HttpResponse::Created().json(series))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetSeriesQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/series",
    params (GetSeriesQuery),
    responses(
        (status = 200, description = "Successfully got the series", body=SeriesAndTitleGroupHierarchyLite),
    )
)]
pub async fn get_series(
    arc: web::Data<Arcadia>,
    query: web::Query<GetSeriesQuery>,
) -> Result<HttpResponse> {
    let series = find_series(&arc.pool, &query.id).await?;

    Ok(HttpResponse::Ok().json(series))
}
