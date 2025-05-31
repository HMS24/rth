use crate::tasks::cron_job::CronJob;
use chrono::Local;
use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobSchedulerError;

pub struct FiveSecondsTask;

impl CronJob for FiveSecondsTask {
    fn create() -> Result<Job, JobSchedulerError> {
        Job::new_async("*/5 * * * * *", |uuid, _lock| {
            Box::pin(async move {
                let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                println!("[{}] I run every 5 seconds! Task id: {}", now, uuid);
            })
        })
    }

    fn name() -> &'static str {
        "FiveSecondsTask"
    }
}
