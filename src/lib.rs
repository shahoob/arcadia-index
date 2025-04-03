pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;

pub struct Arcadia {
    pub pool: sqlx::PgPool,
}
