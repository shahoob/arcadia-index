use crate::{
    connection_pool::ConnectionPool,
    models::collage::{Collage, CollageCategory, UserCreatedCollage},
};
use arcadia_common::error::{Error, Result};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_collage(
        &self,
        collage: &UserCreatedCollage,
        user_id: i64,
    ) -> Result<Collage> {
        let created_collage = sqlx::query_as!(
            Collage,
            r#"
                INSERT INTO collage (created_by_id, name, covers, description, tags, category)
                VALUES ($1, $2, $3, $4, $5, $6::collage_category_enum)
                RETURNING id, created_at, created_by_id, name, covers, description, tags,
                                          category as "category: CollageCategory"
            "#,
            user_id,
            collage.name,
            collage.covers,
            collage.description,
            &collage.tags,
            collage.category as _
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateCollage)?;

        Ok(created_collage)
    }
}
