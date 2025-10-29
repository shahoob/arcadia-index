use crate::Arcadia;
use actix_web::{
    web::{Bytes, Data},
    HttpResponse,
};
use arcadia_shared::{
    error::{BackendError, Result},
    tracker::models::peer_update::{Index, PeerUpdate},
};
use arcadia_storage::redis::RedisPoolInterface;
use bincode::config;

pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    bytes: Bytes,
) -> Result<HttpResponse> {
    let config = config::standard();
    let (updates, _): (Vec<(Index, PeerUpdate)>, usize) =
        bincode::decode_from_slice(&bytes, config)
            .map_err(|e| BackendError::DecodingError(e.to_string()))?;

    arc.pool.bulk_upsert_peers(&updates).await?;

    Ok(HttpResponse::Ok().body(""))
}
