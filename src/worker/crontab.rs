
extern crate job_scheduler;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use std::thread;


pub struct Crontab<'a> {
    pub job:JobScheduler<'a>,
}

impl<'a> Crontab<'a> {
    pub fn add_job(&mut self,job:Job<'a>) {
        &self.job.add(job);
    }

    pub fn start(&mut self) {
        loop {
            &self.job.tick();
            std::thread::sleep(Duration::from_millis(500));
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_job() {
        let mut crontab = Crontab {
            job: JobScheduler::new(),
        };
        crontab.add_job(
            Job::new("* * * * * *".parse().unwrap(), || {
                println!("1");
            })
        );
        crontab.add_job(
            Job::new("1/5 * * * * *".parse().unwrap(), || {
                println!("5");
            })
        );
        crontab.start();
        // start_job();
    }

}
