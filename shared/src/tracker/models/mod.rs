use std::hash::Hash;

use ringmap::RingMap;

pub mod env;
pub mod infohash_2_id;
pub mod passkey_2_id;
pub mod peer;
pub mod peer_id;
pub mod torrent;
pub mod user;
pub mod user_update;

#[derive(Debug)]
pub struct Queue<K, V> {
    records: RingMap<K, V>,
}

pub trait Mergeable {
    /// Merge an existing record with a new record
    fn merge(&mut self, new: &Self);
}

impl<K, V> Default for Queue<K, V>
where
    K: Hash + Eq + Ord,
    V: Clone + Mergeable,
{
    fn default() -> Self {
        Self {
            records: RingMap::new(),
        }
    }
}

impl<K, V> Queue<K, V>
where
    K: Hash + Eq + Ord,
    V: Clone + Mergeable,
{
    /// Upsert a single update into the queue
    pub fn upsert(&mut self, key: K, value: V) {
        self.records
            .entry(key)
            .and_modify(|update| update.merge(&value))
            .or_insert(value);
    }
}
