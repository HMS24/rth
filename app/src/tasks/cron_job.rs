use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobSchedulerError;

pub trait CronJob {
    fn create() -> Result<Job, JobSchedulerError>;
}
