use crate::{
    Arcadia, Result,
    models::{series::UserCreatedSeries, user::User},
    repositories::series_repository::{create_series, find_series},
};
use actix_web::{HttpResponse, web};
use serde::Deserialize;

pub async fn add_series(
    serie: web::Json<UserCreatedSeries>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let series = create_series(&arc.pool, &serie, &current_user).await?;

    Ok(HttpResponse::Created().json(series))
}

#[derive(Debug, Deserialize)]
pub struct GetSeriesQuery {
    id: i64,
}

pub async fn get_series(
    arc: web::Data<Arcadia>,
    query: web::Query<GetSeriesQuery>,
) -> Result<HttpResponse> {
    let series = find_series(&arc.pool, &query.id).await?;

    Ok(HttpResponse::Ok().json(series))
}
