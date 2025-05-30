use chrono::Local;
use tokio_cron_scheduler::Job;

pub trait CronJob {
    fn create() -> Job;
}

pub struct FiveSecondsTask;

impl CronJob for FiveSecondsTask {
    fn create() -> Job {
        Job::new("*/5 * * * * *", |_uuid, _l| {
            let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            println!("[{}] I run every 5 seconds!", now);
        })
        .unwrap()
    }
}
