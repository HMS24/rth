use rth::tasks;

use crate::tasks::cron_job::CronJob;
use crate::tasks::five_seconds_task::FiveSecondsTask;
use chrono::Local;
use tokio_cron_scheduler::{JobScheduler, JobSchedulerError};

pub struct Scheduler {
    scheduler: JobScheduler,
}

impl Scheduler {
    pub async fn new() -> Result<Self, JobSchedulerError> {
        Ok(Self {
            scheduler: JobScheduler::new().await?,
        })
    }

    pub async fn start(self) -> Result<(), JobSchedulerError> {
        self.scheduler.start().await?;
        Ok(())
    }

    pub async fn register_job<T: CronJob>(&self) -> Result<(), JobSchedulerError> {
        self.scheduler.add(T::create()).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    println!(
        "[{}] Starting the scheduler...",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );

    let scheduler = Scheduler::new().await?;

    scheduler.register_job::<FiveSecondsTask>().await?;
    scheduler.start().await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
