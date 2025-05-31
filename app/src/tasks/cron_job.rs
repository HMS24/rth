use tokio_cron_scheduler::Job;

pub trait CronJob {
    fn create() -> Job;
}
