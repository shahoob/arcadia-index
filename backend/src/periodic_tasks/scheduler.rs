use tokio_cron_scheduler::{Job, JobScheduler}; // Import anyhow::Result

async fn testt() {
    println!("test");
}

pub async fn run_periodic_tasks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sched = JobScheduler::new().await?;

    let job1 = Job::new_async("* * * * * *", |_uuid, _l| Box::pin(testt()))?;
    sched.add(job1).await?;

    sched.start().await?;

    Ok(())
}
