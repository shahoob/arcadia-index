use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct StaffPm {
    pub id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub subject: String,
    pub created_by_id: i64,
    pub resolved: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct StaffPmMessage {
    pub id: i64,
    pub staff_pm_id: i64,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub created_by_id: i64,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedStaffPm {
    pub subject: String,
    pub first_message: UserCreatedStaffPmMessage,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedStaffPmMessage {
    pub staff_pm_id: i64,
    pub content: String,
}
