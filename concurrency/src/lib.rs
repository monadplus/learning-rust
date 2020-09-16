// Using Message Passing to Transfer Data Between Threads

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn message_passing() {
    // let (tx, rx) = mpsc::channel(); // multiple producer single consumer
    //
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); // send() takes ownership of the value
    //                            //println!("val is {}", val);
    // });
    //
    // let received = rx.recv().unwrap(); // blocks the thread
    // println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Blocking
    for received in rx {
        println!("Got: {}", received);
    }
}
