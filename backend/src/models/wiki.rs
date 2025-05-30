// use chrono::{DateTime, Local};
// use serde::{Deserialize, Serialize};
// use sqlx::prelude::FromRow;
// use utoipa::ToSchema;

// #[derive(Debug, Serialize, Deserialize, FromRow, Default, ToSchema)]
// pub struct WikiArticle {
//     pub id: i64,
//     #[schema(value_type = String, format = DateTime)]
//     pub created_at: DateTime<Local>,
//     pub created_by_id: i64,
//     #[schema(value_type = String, format = DateTime)]
//     pub updated_at: DateTime<Local>,
//     pub updated_by_id: i64,
//     pub body: String,
// }
