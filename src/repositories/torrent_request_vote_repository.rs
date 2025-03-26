use crate::models::{
    torrent_request_vote::{TorrentRequestVote, UserCreatedTorrentRequestVote},
    user::User,
};
use actix_web::web;
use sqlx::PgPool;
use std::error::Error;

pub async fn create_torrent_request_vote(
    pool: &web::Data<PgPool>,
    torrent_request_vote: &UserCreatedTorrentRequestVote,
    current_user: &User,
) -> Result<TorrentRequestVote, Box<dyn Error>> {
    if current_user.bonus_points - torrent_request_vote.bounty_bonus_points < 0 {
        return Err(format!("you do not have enough bonus points to put this bounty").into());
    }
    if current_user.uploaded - torrent_request_vote.bounty_upload < 0 {
        return Err(format!("you do not have enough upload to put this bounty").into());
    }
    // TODO config: check if the bounties are above the minimum set in the config
    // TODO config: check if the user's ratio stays above the minimum ratio set in the config (after the uploaded amount changes)

    let create_torrent_request_vote_query = r#"
WITH inserted_vote AS (
    INSERT INTO torrent_request_votes (torrent_request_id, created_by_id, 
        bounty_upload, bounty_bonus_points) 
    VALUES ($1, $2, $3, $4)
    RETURNING *
),
updated_request AS (
    UPDATE torrent_requests tr
    SET 
        bounty_upload = tr.bounty_upload + $3,
        bounty_bonus_points = tr.bounty_bonus_points + $4
    WHERE tr.id = $1
),
updated_user AS (
    UPDATE users u
    SET 
        uploaded = u.uploaded - $3,
        bonus_points = u.bonus_points - $4
    WHERE u.id = (SELECT created_by_id FROM inserted_vote)
)
SELECT 
    inserted_vote.* 
FROM inserted_vote;"#;

    let created_torrent_request_vote =
        sqlx::query_as::<_, TorrentRequestVote>(create_torrent_request_vote_query)
            .bind(&torrent_request_vote.torrent_request_id)
            .bind(&current_user.id)
            .bind(&torrent_request_vote.bounty_upload)
            .bind(&torrent_request_vote.bounty_bonus_points)
            .fetch_one(pool.get_ref())
            .await;

    match created_torrent_request_vote {
        Ok(_) => Ok(created_torrent_request_vote.unwrap()),
        Err(e) => {
            println!("{:#?}", e);
            match e {
                sqlx::Error::Database(db_error) => db_error.message().to_string(),
                _ => e.to_string(),
            };
            Err(format!("could not create torrent request vote").into())
        }
    }
}
