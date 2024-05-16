use std::time::Duration;
use std::sync::mpsc;
use std::thread;


// sync vs async chennal in Rust.. 
// sync chennal if we are consume it will get blocked..



fn main(){
    

    // let (sender, recevier) = mpsc::sync_channel(1); // bound 

    let (sender, recevier) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..3 {
            sender.send("Hello from child thread.").unwrap();
            println!("Sent Message {}", i);
        }
    });


    thread::sleep(Duration::from_secs(3));


    for msg in recevier {
        println!("Message Recevied: {}", msg);
    }


}