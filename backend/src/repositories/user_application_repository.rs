use sqlx::PgPool;

use crate::{
    Error, Result,
    models::user_application::{
        UserApplication, UserApplicationStatus, UserCreatedUserApplication,
    },
};

pub async fn create_user_application(
    pool: &PgPool,
    application: &UserCreatedUserApplication,
) -> Result<UserApplication> {
    let gift = sqlx::query_as!(
        UserApplication,
        r#"
            INSERT INTO user_applications (body, referral, email, staff_note, status, invitation_id)
            VALUES ($1, $2, $3, '', 'pending', NULL)
            RETURNING id, created_at, body, email, referral, staff_note,
                      status as "status: UserApplicationStatus", invitation_id
        "#,
        application.body,
        application.referral,
        application.email
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateUserApplication)?;

    Ok(gift)
}

pub async fn find_user_applications(
    pool: &PgPool,
    limit: i64,
    page: i64,
    status: Option<UserApplicationStatus>,
) -> Result<Vec<UserApplication>> {
    let query = format!(
        r#"
            SELECT id, created_at, body, email, referral, staff_note,
                   status::user_application_status_enum as status, invitation_id
            FROM user_applications ua
            WHERE $1 IS NULL OR ua.status = $1::user_application_status_enum
            ORDER BY created_at DESC
            LIMIT {limit} OFFSET $3
        "#
    );

    let applications = sqlx::query_as::<_, UserApplication>(&query)
        .bind(status)
        .bind(limit)
        .bind((page - 1) * limit)
        .fetch_all(pool)
        .await
        .map_err(Error::CouldNotGetUserApplications)?;

    Ok(applications)
}

pub async fn update_user_application_status(
    pool: &PgPool,
    application_id: i64,
    status: UserApplicationStatus,
    invitation_id: Option<i64>,
) -> Result<UserApplication> {
    let application = sqlx::query_as::<_, UserApplication>(
        r#"
            UPDATE user_applications
            SET status = $3::user_application_status_enum, invitation_id = $2
            WHERE id = $1
            RETURNING id, created_at, body, email, referral, staff_note,
                      status::user_application_status_enum as status, invitation_id
        "#,
    )
    .bind(application_id)
    .bind(invitation_id)
    .bind(status)
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotUpdateUserApplication)?;

    Ok(application)
}
