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

pub async fn insert_or_update_peer(
    pool: &PgPool,
    torrent_id: &i64,
    peer_id: &[u8; 20],
    ip: &IpNetwork,
    port: u16,
    user_id: &i64,
) {
    sqlx::query!(
        r#"
          INSERT INTO peers(torrent_id, peer_id, ip, port, user_id) VALUES ($1, $2, $3, $4, $5)
          ON CONFLICT (torrent_id, peer_id, ip, port) DO UPDATE
          SET
            last_seen_at = CURRENT_TIMESTAMP
        "#,
        torrent_id,
        peer_id,
        ip,
        port as i32,
        user_id
    )
    .execute(pool)
    .await
    .expect("failed");
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
