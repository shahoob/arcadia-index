use crate::env::Env;
use std::ops::Deref;

pub mod api_doc;
pub mod env;
pub mod middleware;
pub mod routes;

pub struct Tracker {
    env: Env,
}

impl Deref for Tracker {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl Tracker {
    pub fn new(env: Env) -> Self {
        Self { env }
    }
}
