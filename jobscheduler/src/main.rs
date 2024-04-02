use job_scheduler::{JobScheduler, Job};

fn main() {


    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("I get executed every 10 seconds!");
    }));

    loop {
        sched.tick();
    }

}
