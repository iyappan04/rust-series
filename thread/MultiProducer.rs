use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main(){


    let (sender,receiver) = mpsc::channel();

    let messages = vec!["Hello", "Rust", "Programmer"];

    let cloned_sender = sender.clone();

    thread::spawn(move || {
        for msg in messages { 
            let formatted_msg = format!("Message : {} from thread 1", msg);
            cloned_sender.send(formatted_msg).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });

    let dummy_messages = vec!["Rust", "Is", "Fast"];

    thread::spawn(move || {
        for msg in dummy_messages { 
            let formatted_msg = format!("Message : {} from thread 2", msg);
            sender.send(formatted_msg).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });
 
    // thread::spawn(move || {
    //     for msg in messages { 
    //         sender.send(msg).unwrap();
    //         thread::sleep(Duration::from_millis(2000));
    //     }
    // });

    for msg in receiver {
        println!("{}", msg);
    }


}