use crate::connection_pool::ConnectionPool;
use arcadia_common::error::Result;
use arcadia_shared::tracker::models::{
    infohash_2_id, passkey_2_id, peer,
    torrent::{self, InfoHash, Torrent},
    user::{self, Passkey, User},
};
use indexmap::IndexMap;
use std::borrow::Borrow;

// This file contains functions for Arcadia's tracker
// but not necessarily related to the tracker itself directly

#[derive(Debug)]
pub struct DBImportTorrent {
    pub id: i32,
    pub upload_factor: i16,
    pub download_factor: i16,
    pub seeders: i64,
    pub leechers: i64,
    pub times_completed: i32,
    // somehow sqlx can't see that the returned value is alaways not null
    pub is_deleted: Option<bool>,
}

#[derive(Debug)]
pub struct DBImportUser {
    pub id: i32,
    pub passkey: Passkey,
    pub num_seeding: i32,
    pub num_leeching: i32,
}

#[derive(Debug)]
pub struct DBImportPasskey2Id {
    pub id: i32,
    pub passkey: Passkey,
}

#[derive(Debug)]
pub struct DBImportInfohash2Id {
    pub id: i32,
    pub info_hash: InfoHash,
}

impl ConnectionPool {
    pub async fn find_users(&self) -> Result<user::Map> {
        let rows = sqlx::query_as!(
            DBImportUser,
            r#"
                SELECT
                    id,
                    passkey as "passkey: Passkey",
                    0::INT AS "num_seeding!",
                    0::INT AS "num_leeching!"
                FROM users
            "#
        )
        .fetch_all(self.borrow())
        .await
        .expect("could not get users");

        let mut map: user::Map = user::Map(IndexMap::with_capacity(rows.len()));
        for r in rows {
            let user = User {
                num_seeding: r.num_seeding as u32,
                num_leeching: r.num_leeching as u32,
            };
            map.insert(r.id as u32, user);
        }

        Ok(map)
    }

    pub async fn find_torrents(&self) -> Result<torrent::Map> {
        let rows = sqlx::query_as!(
            DBImportTorrent,
            r#"
            SELECT
                id,
                upload_factor,
                download_factor,
                seeders,
                leechers,
                times_completed,
                CASE
                    WHEN deleted_at IS NOT NULL THEN TRUE
                    ELSE FALSE
                END AS is_deleted
            FROM torrents
            "#
        )
        .fetch_all(self.borrow())
        .await
        .expect("could not get torrents");

        let mut map: torrent::Map = torrent::Map(IndexMap::with_capacity(rows.len()));
        for r in rows {
            let torrent = Torrent {
                upload_factor: r.upload_factor,
                download_factor: r.download_factor,
                seeders: r.seeders as u32,
                leechers: r.leechers as u32,
                times_completed: r.times_completed as u32,
                is_deleted: r.is_deleted.unwrap_or(false),
                peers: peer::Map::new(),
            };
            map.insert(r.id as u32, torrent);
        }

        Ok(map)
    }

    pub async fn find_passkeys_2_ids(&self) -> Result<passkey_2_id::Map> {
        let rows = sqlx::query_as!(
            DBImportPasskey2Id,
            r#"
                    SELECT
                        id,
                        passkey as "passkey: Passkey"
                    FROM users
                    WHERE banned = FALSE
                "#
        )
        .fetch_all(self.borrow())
        .await
        .expect("could not get passkeys2ids");

        let mut map: passkey_2_id::Map = passkey_2_id::Map(IndexMap::with_capacity(rows.len()));
        for r in rows {
            map.insert(r.passkey, r.id as u32);
        }

        Ok(map)
    }

    pub async fn find_infohashes_2_ids(&self) -> Result<infohash_2_id::Map> {
        let rows = sqlx::query_as!(
            DBImportInfohash2Id,
            r#"
                    SELECT
                        id,
                        info_hash as "info_hash: InfoHash"
                    FROM torrents
                "#
        )
        .fetch_all(self.borrow())
        .await
        .expect("could not get infohashes2ids");

        let mut map: infohash_2_id::Map = infohash_2_id::Map(IndexMap::with_capacity(rows.len()));
        for r in rows {
            map.insert(r.info_hash, r.id as u32);
        }

        Ok(map)
    }
}
