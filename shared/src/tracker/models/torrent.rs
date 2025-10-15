#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, bincode::Encode, bincode::Decode)]
pub struct InfoHash(pub [u8; 20]);

#[derive(Debug, Clone, bincode::Encode, bincode::Decode, PartialEq)]
pub struct Torrent {
    pub id: u32,
    pub info_hash: InfoHash,
    pub upload_factor: f64,
    pub download_factor: f64,
    pub seeders: i64,
    pub leechers: i64,
    pub completed: i64,
}
