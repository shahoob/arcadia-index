use crate::{
    connection_pool::ConnectionPool,
    models::staff_pm::{StaffPm, StaffPmMessage, UserCreatedStaffPm, UserCreatedStaffPmMessage},
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_staff_pm(
        &self,
        conversation: &mut UserCreatedStaffPm,
        current_user_id: i64,
    ) -> Result<StaffPm> {
        let created_conversation = sqlx::query_as!(
            StaffPm,
            r#"
				INSERT INTO staff_pms (subject, created_by_id)
				VALUES ($1, $2)
				RETURNING *
			"#,
            conversation.subject,
            current_user_id,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateConversation)?;

        conversation.first_message.staff_pm_id = created_conversation.id;
        self.create_staff_pm_message(&conversation.first_message, current_user_id)
            .await?;

        Ok(created_conversation)
    }

    pub async fn create_staff_pm_message(
        &self,
        message: &UserCreatedStaffPmMessage,
        current_user_id: i64,
    ) -> Result<StaffPmMessage> {
        let message = sqlx::query_as!(
            StaffPmMessage,
            r#"
				INSERT INTO staff_pm_messages (staff_pm_id, created_by_id, content)
				VALUES ($1, $2, $3)
				RETURNING *
			"#,
            message.staff_pm_id,
            current_user_id,
            message.content,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateConversation)?;

        Ok(message)
    }

    pub async fn resolve_staff_pm(
        &self,
        staff_pm_id: i64,
        _current_user_id: i64,
    ) -> Result<StaffPm> {
        let updated = sqlx::query_as!(
            StaffPm,
            r#"
				UPDATE staff_pms
				SET resolved = TRUE
				WHERE id = $1
				RETURNING *
			"#,
            staff_pm_id,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateConversation)?;

        Ok(updated)
    }

    pub async fn list_staff_pms(&self, current_user_id: i64, is_staff: bool) -> Result<Value> {
        let row = sqlx::query_unchecked!(
            r#"
			SELECT
				COALESCE(
					jsonb_agg(
						jsonb_build_object(
							'id', sp.id,
							'created_at', sp.created_at,
							'subject', sp.subject,
							'created_by_id', sp.created_by_id,
							'resolved', sp.resolved,
							'last_message', jsonb_build_object(
								'created_at', lm.created_at,
								'created_by_id', lm.created_by_id
							)
						)
						ORDER BY lm.created_at DESC
					),
					'[]'::jsonb
				) AS "staff_pms_json: serde_json::Value"
			FROM staff_pms sp
			JOIN LATERAL (
				SELECT created_at, created_by_id
				FROM staff_pm_messages
				WHERE staff_pm_id = sp.id
				ORDER BY created_at DESC
				LIMIT 1
			) lm ON TRUE
			WHERE ($2)::bool OR sp.created_by_id = $1;
			"#,
            current_user_id,
            is_staff,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindConversations)?;

        Ok(row.staff_pms_json.unwrap())
    }

    pub async fn get_staff_pm(
        &self,
        staff_pm_id: i64,
        current_user_id: i64,
        is_staff: bool,
    ) -> Result<Value> {
        let row = sqlx::query_unchecked!(
            r#"
			SELECT
				json_build_object(
					'id', sp.id,
					'created_at', sp.created_at,
					'subject', sp.subject,
					'created_by_id', sp.created_by_id,
					'resolved', sp.resolved,
					'messages', (
						SELECT json_agg(json_build_object(
							'id', m.id,
							'created_at', m.created_at,
							'created_by_id', m.created_by_id,
							'content', m.content
						) ORDER BY m.created_at ASC)
						FROM staff_pm_messages m
						WHERE m.staff_pm_id = sp.id
					)
				) AS "conversation: serde_json::Value"
			FROM staff_pms sp
			WHERE sp.id = $1 AND (($3)::bool OR sp.created_by_id = $2);
			"#,
            staff_pm_id,
            current_user_id,
            is_staff,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindConversation)?;

        Ok(row.conversation.unwrap())
    }
}
