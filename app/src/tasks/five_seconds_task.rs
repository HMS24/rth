use crate::tasks::cron_job::CronJob;
use chrono::Local;
use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobSchedulerError;

pub struct FiveSecondsTask;

impl CronJob for FiveSecondsTask {
    fn create() -> Result<Job, JobSchedulerError> {
        Job::new("*/5 * * * * *", |uuid, _lock| {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            println!("Task: {}, Five seconds task executed at: {}", uuid, now);
        })
    }

    fn name() -> &'static str {
        "FiveSecondsTask"
    }
}
