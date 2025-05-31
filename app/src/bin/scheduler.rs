use rth::tasks;

use crate::tasks::cron_job::CronJob;
use crate::tasks::five_seconds_task::FiveSecondsTask;
use crate::tasks::ten_seconds_task::TenSecondsTask;
use tokio::signal;
use tokio_cron_scheduler::{JobScheduler, JobSchedulerError};
use tracing::{error, info};

pub struct Scheduler {
    scheduler: JobScheduler,
}

impl Scheduler {
    pub async fn new() -> Result<Self, JobSchedulerError> {
        Ok(Self {
            scheduler: JobScheduler::new().await?,
        })
    }

    pub async fn start(&mut self) -> Result<(), JobSchedulerError> {
        self.scheduler.start().await?;
        Ok(())
    }

    pub async fn register_task<T: CronJob>(&self) -> Result<(), JobSchedulerError> {
        let task = T::create()?;
        self.scheduler.add(task).await?;

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), JobSchedulerError> {
        self.scheduler.shutdown().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting the scheduler...");

    let mut scheduler = Scheduler::new().await?;

    scheduler.register_task::<FiveSecondsTask>().await?;
    scheduler.register_task::<TenSecondsTask>().await?;

    scheduler.start().await?;

    match signal::ctrl_c().await {
        Ok(()) => {
            scheduler.shutdown().await?;

            info!("Scheduler shutdown completed");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    Ok(())
}
