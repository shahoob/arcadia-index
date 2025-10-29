use crate::connection_pool::ConnectionPool;
use arcadia_common::error::Result;
use arcadia_shared::tracker::models::{
    infohash_2_id, passkey_2_id,
    peer::{self, Peer},
    peer_id::PeerId,
    peer_update::{self, PeerUpdate},
    torrent::{self, InfoHash, Torrent},
    torrent_update::{self, TorrentUpdate},
    user::{self, Passkey, User},
    user_update::{self, UserUpdate},
};
use chrono::{DateTime, Utc};
use futures_util::TryStreamExt;
use indexmap::IndexMap;
use sqlx::types::ipnetwork::IpNetwork;
use std::{borrow::Borrow, net::IpAddr};

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
    pub is_deleted: bool,
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
                END AS "is_deleted!"
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
                is_deleted: r.is_deleted,
                peers: peer::Map::new(),
            };
            map.insert(r.id as u32, torrent);
        }

        // Load peers into each torrent
        let mut peers = sqlx::query!(
            r#"
                SELECT
                    peers.ip as "ip_address: IpAddr",
                    peers.user_id as "user_id",
                    peers.torrent_id as "torrent_id",
                    peers.port as "port",
                    peers.seeder as "is_seeder: bool",
                    peers.active as "is_active: bool",
                    peers.updated_at as "updated_at: DateTime<Utc>",
                    peers.uploaded as "uploaded",
                    peers.downloaded as "downloaded",
                    peers.peer_id as "peer_id: PeerId"
                FROM
                    peers
            "#
        )
        .fetch(self.borrow());

        while let Some(peer) = peers.try_next().await.expect("Failed loading peers.") {
            map.entry(peer.torrent_id as u32).and_modify(|torrent| {
                torrent.peers.insert(
                    peer::Index {
                        user_id: peer.user_id as u32,
                        peer_id: peer.peer_id,
                    },
                    Peer {
                        ip_address: peer.ip_address,
                        port: peer.port as u16,
                        is_seeder: peer.is_seeder,
                        is_active: peer.is_active,
                        has_sent_completed: false,
                        updated_at: peer
                            .updated_at
                            .expect("Peer with a null updated_at found in database."),
                        uploaded: peer.uploaded as u64,
                        downloaded: peer.downloaded as u64,
                    },
                );
            });
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

    pub async fn bulk_update_users(
        &self,
        updates: &Vec<(user_update::Index, UserUpdate)>,
    ) -> arcadia_shared::error::Result<()> {
        if updates.is_empty() {
            return Ok(());
        }

        let mut user_ids = Vec::new();
        let mut uploaded_deltas = Vec::new();
        let mut downloaded_deltas = Vec::new();
        let mut real_uploaded_deltas = Vec::new();
        let mut real_downloaded_deltas = Vec::new();

        for (index, update) in updates {
            user_ids.push(index.user_id as i32);
            uploaded_deltas.push(update.uploaded_delta as i64);
            downloaded_deltas.push(update.downloaded_delta as i64);
            real_uploaded_deltas.push(update.real_uploaded_delta as i64);
            real_downloaded_deltas.push(update.real_downloaded_delta as i64);
        }

        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET
                    uploaded = uploaded + updates.uploaded_delta,
                    downloaded = downloaded + updates.downloaded_delta,
                    real_uploaded = real_uploaded + updates.uploaded_delta,
                    real_downloaded = real_downloaded + updates.downloaded_delta
                FROM (
                    SELECT * FROM unnest($1::int[], $2::bigint[], $3::bigint[], $4::bigint[], $5::bigint[]) AS
                    t(user_id, uploaded_delta, downloaded_delta, real_uploaded_delta, real_downloaded_delta)
                ) AS updates
                WHERE users.id = updates.user_id
            "#,
            &user_ids,
            &uploaded_deltas,
            &downloaded_deltas,
            &real_uploaded_deltas,
            &real_downloaded_deltas
        )
        .execute(self.borrow())
        .await.map_err(|e|arcadia_shared::error::BackendError::DatabseError(e.to_string()))?;

        Ok(())
    }

    pub async fn bulk_update_torrents(
        &self,
        updates: &Vec<(torrent_update::Index, TorrentUpdate)>,
    ) -> arcadia_shared::error::Result<()> {
        if updates.is_empty() {
            return Ok(());
        }

        let mut torrent_ids = Vec::new();
        let mut seeder_deltas = Vec::new();
        let mut leecher_deltas = Vec::new();
        let mut times_completed_deltas = Vec::new();

        for (index, update) in updates {
            torrent_ids.push(index.torrent_id as i32);
            seeder_deltas.push(update.seeder_delta as i64);
            leecher_deltas.push(update.leecher_delta as i64);
            times_completed_deltas.push(update.times_completed_delta as i64);
        }

        let _ = sqlx::query!(
            r#"
                UPDATE torrents
                SET
                    seeders = seeders + updates.seeder_delta,
                    leechers = leechers + updates.leecher_delta,
                    times_completed = times_completed + updates.times_completed_delta
                FROM (
                    SELECT * FROM unnest($1::int[], $2::bigint[], $3::bigint[], $4::bigint[]) AS
                    t(torrent_id, seeder_delta, leecher_delta, times_completed_delta)
                ) AS updates
                WHERE torrents.id = updates.torrent_id
            "#,
            &torrent_ids,
            &seeder_deltas,
            &leecher_deltas,
            &times_completed_deltas,
        )
        .execute(self.borrow())
        .await
        .map_err(|e| arcadia_shared::error::BackendError::DatabseError(e.to_string()))?;

        Ok(())
    }

    pub async fn bulk_upsert_peers(
        &self,
        updates: &Vec<(peer_update::Index, PeerUpdate)>,
    ) -> arcadia_shared::error::Result<u64> {
        if updates.is_empty() {
            return Ok(0);
        }

        let mut user_ids: Vec<i32> = Vec::with_capacity(updates.len());
        let mut torrent_ids: Vec<i32> = Vec::with_capacity(updates.len());
        let mut peer_ids: Vec<Vec<u8>> = Vec::with_capacity(updates.len());
        let mut ips: Vec<IpNetwork> = Vec::with_capacity(updates.len());
        let mut ports: Vec<i32> = Vec::with_capacity(updates.len());
        let mut agents: Vec<String> = Vec::with_capacity(updates.len());
        let mut uploadeds: Vec<i64> = Vec::with_capacity(updates.len());
        let mut downloadeds: Vec<i64> = Vec::with_capacity(updates.len());
        let mut lefts: Vec<i64> = Vec::with_capacity(updates.len());
        let mut actives: Vec<bool> = Vec::with_capacity(updates.len());
        let mut seeders: Vec<bool> = Vec::with_capacity(updates.len());
        let mut created_ats: Vec<DateTime<Utc>> = Vec::with_capacity(updates.len());
        let mut updated_ats: Vec<DateTime<Utc>> = Vec::with_capacity(updates.len());

        for (index, update) in updates {
            user_ids.push(index.user_id as i32);
            torrent_ids.push(index.torrent_id as i32);
            peer_ids.push(index.peer_id.to_vec());
            ips.push(IpNetwork::from(update.ip));
            ports.push(update.port as i32);
            agents.push(update.agent.clone());
            uploadeds.push(update.uploaded as i64);
            downloadeds.push(update.downloaded as i64);
            lefts.push(update.left as i64);
            actives.push(update.is_active);
            seeders.push(update.is_seeder);
            created_ats.push(update.created_at);
            updated_ats.push(update.updated_at);
        }

        let result = sqlx::query!(
            r#"
                INSERT INTO peers (
                    peer_id,
                    ip,
                    port,
                    agent,
                    uploaded,
                    downloaded,
                    "left",
                    active,
                    seeder,
                    created_at,
                    updated_at,
                    torrent_id,
                    user_id
                )
                SELECT
                    t.peer_id,
                    t.ip,
                    t.port,
                    t.agent,
                    t.uploaded,
                    t.downloaded,
                    t."left",
                    t.active,
                    t.seeder,
                    -- stored as timestamp without time zone in DB
                    (t.created_at AT TIME ZONE 'UTC')::timestamp,
                    (t.updated_at AT TIME ZONE 'UTC')::timestamp,
                    t.torrent_id,
                    t.user_id
                FROM (
                    SELECT * FROM unnest(
                        $1::bytea[],
                        $2::inet[],
                        $3::int[],
                        $4::varchar[],
                        $5::bigint[],
                        $6::bigint[],
                        $7::bigint[],
                        $8::boolean[],
                        $9::boolean[],
                        $10::timestamptz[],
                        $11::timestamptz[],
                        $12::int[],
                        $13::int[]
                    ) AS t(
                        peer_id,
                        ip,
                        port,
                        agent,
                        uploaded,
                        downloaded,
                        "left",
                        active,
                        seeder,
                        created_at,
                        updated_at,
                        torrent_id,
                        user_id
                    )
                ) AS t
                ON CONFLICT (user_id, torrent_id, peer_id) DO UPDATE SET
                    ip = EXCLUDED.ip,
                    port = EXCLUDED.port,
                    agent = EXCLUDED.agent,
                    uploaded = EXCLUDED.uploaded,
                    downloaded = EXCLUDED.downloaded,
                    "left" = EXCLUDED."left",
                    active = EXCLUDED.active,
                    seeder = EXCLUDED.seeder,
                    updated_at = EXCLUDED.updated_at
            "#,
            &peer_ids,
            &ips,
            &ports,
            &agents,
            &uploadeds,
            &downloadeds,
            &lefts,
            &actives,
            &seeders,
            &created_ats,
            &updated_ats,
            &torrent_ids,
            &user_ids
        )
        .execute(self.borrow())
        .await
        .map_err(|e| arcadia_shared::error::BackendError::DatabseError(e.to_string()))?;

        Ok(result.rows_affected())
    }

    pub async fn bulk_delete_peers(
        &self,
        peers: &Vec<peer_update::Index>,
    ) -> arcadia_shared::error::Result<()> {
        if peers.is_empty() {
            return Ok(());
        }

        let mut user_ids: Vec<i32> = Vec::with_capacity(peers.len());
        let mut torrent_ids: Vec<i32> = Vec::with_capacity(peers.len());
        let mut peer_ids: Vec<Vec<u8>> = Vec::with_capacity(peers.len());

        for index in peers {
            user_ids.push(index.user_id as i32);
            torrent_ids.push(index.torrent_id as i32);
            peer_ids.push(index.peer_id.to_vec());
        }

        let _ = sqlx::query!(
            r#"
                DELETE FROM peers
                WHERE (user_id, torrent_id, peer_id) IN (
                    SELECT t.user_id, t.torrent_id, t.peer_id
                    FROM (
                        SELECT * FROM unnest(
                            $1::int[],
                            $2::int[],
                            $3::bytea[]
                        ) AS t(user_id, torrent_id, peer_id)
                    ) AS t
                )
            "#,
            &user_ids,
            &torrent_ids,
            &peer_ids
        )
        .execute(self.borrow())
        .await
        .map_err(|e| arcadia_shared::error::BackendError::DatabseError(e.to_string()))?;

        Ok(())
    }
}
