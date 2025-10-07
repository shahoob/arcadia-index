use indexmap::IndexMap;
use serde::Serialize;

pub use arcadia_shared::tracker::models::user::{Passkey, User};

#[derive(Serialize)]
pub struct Map(IndexMap<u32, User>);
