use std::{
    future::{self, Ready},
    str::FromStr,
};

use crate::{
    announce::{
        error::{AnnounceError, Result},
        models::torrent::{Announce, TorrentEvent},
    },
    services::announce_service::is_torrent_client_allowed,
    Tracker,
};
use actix_web::{
    dev,
    web::{Data, Path},
    FromRequest, HttpRequest, HttpResponse,
};
use arcadia_shared::tracker::models::user::Passkey;

#[derive(Debug)]
pub struct UserAgent(pub String);

impl std::ops::Deref for UserAgent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl FromRequest for UserAgent {
    type Error = AnnounceError;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let agent = req
            .headers()
            .get("User-Agent")
            .ok_or(AnnounceError::NoUserAgent)
            .and_then(|s| {
                let agent_string = s.to_str()?;
                if agent_string.len() > 64 {
                    // Block user agent strings that are too long. (For Database reasons)
                    Err(AnnounceError::UserAgentTooLong)
                } else {
                    Ok(UserAgent(String::from(agent_string)))
                }
            });

        future::ready(agent)
    }
}

#[utoipa::path(
    get,
    operation_id = "Announce",
    tag = "Announce",
    path = "/{passkey}/announce",
    responses(
        (status = 200, description = "Announce"),
    )
)]
pub async fn exec(
    arc: Data<Tracker>,
    passkey: Path<String>,
    agent: UserAgent,
    ann: Announce,
    conn: dev::ConnectionInfo,
    // req: HttpRequest,
) -> Result<HttpResponse> {
    println!("Announce request: {:?}", ann);
    println!("Passkey: {:?}", passkey);
    println!("User-Agent: {:?}", agent);
    println!("Connection info: {:?}", conn);
    println!("Tracker: {:?}", arc);
    // let headers = req.headers();
    // if headers.contains_key(ACCEPT_LANGUAGE)
    //     || headers.contains_key(REFERER)
    //     || headers.contains_key(ACCEPT_CHARSET)
    //     // This header check may block Non-bittorrent client `Aria2` to access tracker,
    //     // Because they always add this header which other clients don't have.
    //     //
    //     // See: https://blog.rhilip.info/archives/1010/ ( in Chinese )
    //     || headers.contains_key("want-digest")
    // {
    //     return Err(AnnounceError::AbnormalAccess);
    // }

    if !is_torrent_client_allowed(&ann.peer_id, &arc.env.allowed_torrent_clients.clients) {
        return Err(AnnounceError::TorrentClientNotInWhitelist);
    }

    let passkey = Passkey::from_str(&passkey).or(Err(AnnounceError::InvalidPasskey))?;
    // Validate passkey
    let user_id = arc
        .passkey2id
        .read()
        .get(&passkey)
        .ok_or(AnnounceError::PasskeyNotFound)
        .cloned();

    let user = if let Ok(user_id) = user_id {
        arc.users
            .read()
            .get(&user_id)
            .ok_or(AnnounceError::UserNotFound)
            .cloned()
    } else {
        Err(AnnounceError::UserNotFound)
    };

    // Validate torrent
    let torrent_id_res = arc
        .infohash2id
        .read()
        .get(&ann.info_hash)
        .ok_or(AnnounceError::InfoHashNotFound)
        .cloned();

    // if let Ok(user) = &user {
    //     if let Err(InfoHashNotFound) = torrent_id_res {
    //         tracker.unregistered_info_hash_updates.lock().upsert(
    //             unregistered_info_hash_update::Index {
    //                 user_id: user.id,
    //                 info_hash: queries.info_hash,
    //             },
    //             UnregisteredInfoHashUpdate {
    //                 created_at: now,
    //                 updated_at: now,
    //             },
    //         );
    //     }
    // }

    let torrent_id = torrent_id_res?;

    let mut torrent_guard = arc.torrents.lock();
    let torrent = torrent_guard
        .get_mut(&torrent_id)
        .ok_or(AnnounceError::TorrentNotFound)?;

    if torrent.is_deleted {
        return Err(AnnounceError::TorrentIsDeleted);
    }

    println!("{:?}", torrent);

    Ok(HttpResponse::Ok().json("{}"))
}
