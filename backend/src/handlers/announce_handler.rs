use crate::Arcadia;
use crate::tracker::announce;
use actix_web::{
    FromRequest, HttpRequest, HttpResponse, HttpResponseBuilder, ResponseError, dev, get, web,
};
use serde::Serialize;
use sqlx::types::ipnetwork::IpNetwork;
use std::future::{self, Ready};

trait HttpResponseBuilderExt {
    fn bencode(&mut self, val: impl Serialize) -> HttpResponse;
}

impl HttpResponseBuilderExt for HttpResponseBuilder {
    fn bencode(&mut self, val: impl Serialize) -> HttpResponse {
        match serde_bencode::to_bytes(&val) {
            Ok(data) => self.body(data),
            Err(_) => HttpResponse::BadRequest().body("Failed to bencode"),
        }
    }
}

impl ResponseError for announce::Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().body(format!("{}", self))
    }
}

impl FromRequest for announce::Announce {
    type Error = announce::Error;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let query_string = req.query_string();

        let announce = announce::decode_from_query_str(query_string);

        future::ready(announce)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid passkey")]
    InvalidPassKey,

    #[error("invalid info_hash")]
    InvalidInfoHash,
}

impl actix_web::ResponseError for Error {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        #[derive(Debug, Serialize)]
        struct WrappedError {
            #[serde(rename = "failure reason")]
            failure_reason: String,
        }

        actix_web::HttpResponse::build(self.status_code()).bencode(WrappedError {
            failure_reason: self.to_string(),
        })
    }
}

type Result<T> = std::result::Result<T, Error>;

#[get("/announce/{passkey}")]
async fn handle_announce(
    arc: web::Data<Arcadia>,
    passkey: web::Path<String>,
    ann: announce::Announce,
    conn: dev::ConnectionInfo,
) -> Result<HttpResponse> {
    let passkey = u128::from_str_radix(&passkey, 16).map_err(|_| Error::InvalidPassKey)?;

    let passkey_upper = (passkey >> 64) as i64;
    let passkey_lower = passkey as i64;

    let user = sqlx::query!(
        r#"
            SELECT id, username FROM users
            WHERE (passkey_upper, passkey_lower) = ($1, $2)
        "#,
        passkey_upper,
        passkey_lower
    )
    .fetch_one(&arc.pool)
    .await
    .map_err(|_| Error::InvalidPassKey)?;

    let torrent = sqlx::query!(
        r#"
            SELECT id FROM torrents
            WHERE info_hash = $1
        "#,
        &ann.info_hash
    )
    .fetch_one(&arc.pool)
    .await
    .map_err(|_| Error::InvalidInfoHash)?;

    // TODO check peer id prefix

    let ip = conn
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpNetwork>().ok())
        .unwrap();

    if let Some(announce::TorrentEvent::Stopped) = ann.event {
        sqlx::query!(
            r#"
            DELETE FROM peers WHERE
            (torrent_id, peer_id, ip, port) = ($1, $2, $3, $4)
            "#,
            &torrent.id,
            &ann.peer_id,
            &ip,
            ann.port as i32
        )
        .execute(&arc.pool)
        .await
        .expect("failed");
        //return HttpResponse::Ok().into();
        todo!();
    }

    sqlx::query!(
        r#"
        WITH peer_id AS (
            INSERT INTO peers(torrent_id, peer_id, ip, port) VALUES ($1, $2, $3, $4)
            ON CONFLICT (torrent_id, peer_id, ip, port) DO UPDATE
            SET
              last_seen_at = CURRENT_TIMESTAMP
            RETURNING id
        )
        INSERT INTO user_peers(user_id, peer_id) SELECT $5 AS user_id, peer_id.id AS peer_id FROM peer_id
        ON CONFLICT (user_id, peer_id) DO NOTHING
        "#,
        &torrent.id,
        &ann.peer_id,
        ip,
        ann.port as i32,
        user.id
    )
    .execute(&arc.pool)
    .await
    .expect("failed");

    let peers = sqlx::query!(
        r#"
        SELECT peers.ip AS ip, peers.port AS port
        FROM peers
        JOIN user_peers ON user_peers.peer_id = peers.id
        WHERE
            torrent_id = $1
        AND
            user_peers.user_id != $2
        "#,
        &torrent.id,
        user.id
    )
    .fetch_all(&arc.pool)
    .await
    .expect("failed");

    let peers = peers
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
        .collect::<Vec<_>>();

    let resp = announce::AnnounceResponse {
        peers,
        ..Default::default()
    };

    Ok(HttpResponse::Ok().bencode(resp))
}
