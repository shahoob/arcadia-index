use sqlx::{PgPool, types::ipnetwork::IpNetwork};

use crate::tracker::announce::{self, PeerCompact};

pub async fn remove_peer(
    pool: &PgPool,
    torrent_id: &i64,
    peer_id: &[u8; 20],
    ip: &IpNetwork,
    port: u16,
) {
    sqlx::query!(
        r#"
            DELETE FROM peers WHERE
            (torrent_id, peer_id, ip, port) = ($1, $2, $3, $4)
            "#,
        torrent_id,
        peer_id,
        ip,
        port as i32
    )
    .execute(pool)
    .await
    .expect("failed removing peer from table");
}

// returns uploaded/downloaded before the update
pub async fn insert_or_update_peer(
    pool: &PgPool,
    torrent_id: &i64,
    peer_id: &[u8; 20],
    ip: &IpNetwork,
    port: u16,
    user_id: &i64,
    uploaded: i64,
    downloaded: i64,
) -> (i64, i64) {
    let existing = sqlx::query!(
        r#"
        SELECT real_uploaded, real_downloaded
        FROM peers
        WHERE torrent_id = $1 AND peer_id = $2 AND ip = $3 AND port = $4 AND user_id = $5
        "#,
        torrent_id,
        peer_id,
        ip,
        port as i32,
        user_id
    )
    .fetch_optional(pool)
    .await
    .expect("failed");

    sqlx::query!(
        r#"
        INSERT INTO peers(torrent_id, peer_id, ip, port, user_id, real_uploaded, real_downloaded)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (torrent_id, peer_id, ip, port) DO UPDATE
        SET
            last_seen_at = CURRENT_TIMESTAMP,
            real_uploaded = $6,
            real_downloaded = $7
        "#,
        torrent_id,
        peer_id,
        ip,
        port as i32,
        user_id,
        uploaded,
        downloaded
    )
    .execute(pool)
    .await
    .expect("failed");

    match existing {
        Some(row) => (row.real_uploaded, row.real_downloaded),
        None => (0, 0),
    }
}

pub async fn find_torrent_peers(
    pool: &PgPool,
    torrent_id: &i64,
    user_id: &i64,
) -> Vec<PeerCompact> {
    let peers = sqlx::query!(
        r#"
        SELECT peers.ip AS ip, peers.port AS port
        FROM peers
        WHERE
            torrent_id = $1
        AND
            peers.user_id != $2
        "#,
        torrent_id,
        user_id
    )
    .fetch_all(pool)
    .await
    .expect("failed");

    peers
        .into_iter()
        .map(|p| {
            let std::net::IpAddr::V4(ipv4) = p.ip.ip() else {
                panic!("oops");
            };

            announce::PeerCompact {
                ip: ipv4,
                port: p.port as u16,
            }
        })
        .collect::<Vec<_>>()
}
