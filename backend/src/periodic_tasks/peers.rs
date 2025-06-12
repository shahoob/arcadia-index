use sqlx::PgPool;

use crate::repositories::peer_repository;

pub async fn remove_inactive_peers(pool: PgPool, announce_interval: u32) {
    // adding a 120s grace period in case of network latencies/server load/etc.
    let _ = peer_repository::remove_inactive_peers(&pool, (announce_interval + 120) as f64).await;
}
