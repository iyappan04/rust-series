// MPSC chennal 

// MPSC chennal => Mulitple Producer and Single Consumer

// Pass data between threads 

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main(){


    let (sender,receiver) = mpsc::channel();

    let messages = vec!["Hello", "Rust", "Programmer"];

    // child thread produce the message and main thread consume the messages 

    thread::spawn(move || {
        for msg in messages { 
            sender.send(msg).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });

    for msg in receiver {
        println!("{}", msg);
    }


}