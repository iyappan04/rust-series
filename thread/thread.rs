// threads -> run mutiple task concurrently, (or) simultaneous
use std::thread;
use std::time::Duration;

fn main() {

    let thread1 = thread::spawn(|| {
        for k in 1..11 { 
            println!("In thread1, executing round is {}", k);
            thread::sleep(Duration::from_millis(2000));
        }
    });

    // thread1.join().unwrap(); // it will finish the first thread to finish 

    for i in 1..6 {
        println!("In main, executing round is {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    thread1.join().unwrap(); // it will run concurrently both threads at the same time

}