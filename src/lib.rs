pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OpenSignups {
    Disabled,
    Enabled,
}

pub struct Arcadia {
    pub pool: sqlx::PgPool,
    pub open_signups: OpenSignups,
}

impl Arcadia {
    #[inline]
    pub fn is_open_signups(&self) -> bool {
        self.open_signups == OpenSignups::Enabled
    }
}
