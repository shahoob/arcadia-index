use std::env;

use actix_web::web;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::Arcadia;

use super::torrents::update_torrent_seeders_leechers;

pub async fn run_periodic_tasks(arc: web::Data<Arcadia>) -> Result<(), Box<dyn std::error::Error>> {
    let sched = JobScheduler::new().await?;

    let update_torrent_seeders_leechers_interval = env::var("TASK_UPDATE_TORRENT_SEEDERS_LEECHERS")
        .expect("TASK_UPDATE_TORRENT_SEEDERS_LEECHERS is missing");
    let job1 = match Job::new_async(
        update_torrent_seeders_leechers_interval.as_str(),
        move |_uuid, _l| Box::pin(update_torrent_seeders_leechers(arc.pool.clone())),
    ) {
        Ok(job) => job,
        Err(e) => {
            return Err(format!(
                "Error creating job for TASK_UPDATE_TORRENT_SEEDERS_LEECHERS: {e}"
            )
            .into());
        }
    };
    sched.add(job1).await?;

    //TODO: add clean old peers (in case we didn't receive a stopped event)

    sched.start().await?;

    Ok(())
}
