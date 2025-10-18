use crate::connection_pool::ConnectionPool;
use arcadia_common::error::Result;
use arcadia_shared::tracker::models::{
    torrent::{InfoHash, Torrent},
    user::Passkey,
    user::User,
};
use std::borrow::Borrow;
use std::collections::HashMap;

// This file contains functions for Arcadia's tracker
// but not necessarily related to the tracker itself directly

#[derive(Debug)]
pub struct DBImportTorrent {
    pub id: i32,
    pub upload_factor: f64,
    pub download_factor: f64,
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
    pub async fn find_users(&self) -> Result<HashMap<u32, User>> {
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

        let mut map: HashMap<u32, User> = HashMap::with_capacity(rows.len());
        for r in rows {
            let user = User {
                num_seeding: r.num_seeding as u32,
                num_leeching: r.num_leeching as u32,
            };
            map.insert(r.id as u32, user);
        }

        Ok(map)
    }

    pub async fn find_torrents(&self) -> Result<HashMap<u32, Torrent>> {
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

        let mut map: HashMap<u32, Torrent> = HashMap::with_capacity(rows.len());
        for r in rows {
            let torrent = Torrent {
                upload_factor: r.upload_factor,
                download_factor: r.download_factor,
                seeders: r.seeders,
                leechers: r.leechers,
                times_completed: r.times_completed,
                is_deleted: r.is_deleted.unwrap_or(false),
            };
            map.insert(r.id as u32, torrent);
        }

        Ok(map)
    }

    pub async fn find_passkeys_2_ids(&self) -> Result<HashMap<Passkey, u32>> {
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

        let mut map: HashMap<Passkey, u32> = HashMap::with_capacity(rows.len());
        for r in rows {
            map.insert(r.passkey, r.id as u32);
        }

        Ok(map)
    }

    pub async fn find_infohashes_2_ids(&self) -> Result<HashMap<InfoHash, u32>> {
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

        let mut map: HashMap<InfoHash, u32> = HashMap::with_capacity(rows.len());
        for r in rows {
            map.insert(r.info_hash, r.id as u32);
        }

        Ok(map)
    }
}
