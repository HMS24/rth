use rth::tasks;

use crate::tasks::cron_job::CronJob;
use crate::tasks::five_seconds_task::FiveSecondsTask;
use tokio_cron_scheduler::{JobScheduler, JobSchedulerError};
use tracing::info;

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

    pub async fn register_task<T: CronJob>(&self) -> Result<(), JobSchedulerError> {
        let task = T::create()?;
        self.scheduler.add(task).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting the scheduler...");

    let scheduler = Scheduler::new().await?;

    scheduler.register_task::<FiveSecondsTask>().await?;
    scheduler.start().await?;

    tokio::signal::ctrl_c().await?;

    Ok(())
}
