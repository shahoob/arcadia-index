use crate::{handlers::tracker::binary_response, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;

pub async fn exec<R: RedisPoolInterface + 'static>(arc: Data<Arcadia<R>>) -> Result<HttpResponse> {
    let infohashes_2_ids = arc.pool.find_infohashes_2_ids().await?;
    binary_response(&infohashes_2_ids)
}
