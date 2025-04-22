use crate::repositories::announce_repository::find_torrent_with_id;
use crate::repositories::peer_repository::{
    find_torrent_peers, insert_or_update_peer, remove_peer,
};
use crate::tracker::announce;
use crate::{Arcadia, repositories::announce_repository::find_user_with_passkey};
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

    let user = find_user_with_passkey(&arc.pool, passkey_upper, passkey_lower).await?;

    let torrent = find_torrent_with_id(&arc.pool, &ann.info_hash).await?;

    // TODO check peer id prefix

    let ip = conn
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpNetwork>().ok())
        .unwrap();

    if let Some(announce::TorrentEvent::Stopped) = ann.event {
        remove_peer(&arc.pool, &torrent.id, &ann.peer_id, &ip, ann.port).await;
        //return HttpResponse::Ok().into();
        todo!();
    }

    insert_or_update_peer(
        &arc.pool,
        &torrent.id,
        &ann.peer_id,
        &ip,
        ann.port,
        &user.id,
    )
    .await;

    let peers = find_torrent_peers(&arc.pool, &torrent.id, &user.id).await;

    let resp = announce::AnnounceResponse {
        peers,
        ..Default::default()
    };

    Ok(HttpResponse::Ok().bencode(resp))
}
