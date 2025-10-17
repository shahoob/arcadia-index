use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Invalid infohash.")]
    InfoHash,
}
