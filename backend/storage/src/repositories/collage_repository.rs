use crate::{
    connection_pool::ConnectionPool,
    models::collage::{
        Collage, CollageCategory, CollageEntry, CollageType, UserCreatedCollage,
        UserCreatedCollageEntry,
    },
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
                INSERT INTO collage (created_by_id, name, covers, description, tags, category, collage_type)
                VALUES ($1, $2, $3, $4, $5, $6::collage_category_enum, $7::collage_type_enum)
                RETURNING id, created_at, created_by_id, name, covers, description, tags, collage_type as "collage_type: CollageType",
                                          category as "category: CollageCategory"
            "#,
            user_id,
            collage.name,
            collage.covers,
            collage.description,
            &collage.tags,
            collage.category as _,
            collage.collage_type as _
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateCollage)?;

        Ok(created_collage)
    }

    pub async fn create_collage_entries(
        &self,
        collage_entries: &[UserCreatedCollageEntry],
        user_id: i64,
    ) -> Result<Vec<CollageEntry>> {
        let mut created_entries = Vec::with_capacity(collage_entries.len());

        for entry in collage_entries {
            let created = sqlx::query_as!(
                CollageEntry,
                r#"
                    INSERT INTO collage_entry (
                        created_by_id,
                        artist_id,
                        entity_id,
                        title_group_id,
                        master_group_id,
                        collage_id,
                        note
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    RETURNING *
                "#,
                user_id,
                entry.artist_id,
                entry.entity_id,
                entry.title_group_id,
                entry.master_group_id,
                entry.collage_id,
                entry.note
            )
            .fetch_one(self.borrow())
            .await
            .map_err(|e| Error::CouldNotCreateCollageEntry(e.to_string()))?;

            created_entries.push(created);
        }

        Ok(created_entries)
    }
}
