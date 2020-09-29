use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Using Message Passing to Transfer Data Between Threads
pub fn message_passing() {
    // let (tx, rx) = mpsc::channel(); // multiple producer single consumer
    //
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); // send() takes ownership of the value
    //     //println!("val is {}", val);
    // });
    //
    // let received = rx.recv().unwrap(); // blocks the thread
    // println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
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

//use std::rc::Rc;
use std::sync::{Arc, Mutex};

// Using Mutexes to Allow Access to Data from One Thread at a Time
pub fn mutex() {
    // mutex = mutual exclusion
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // As you might suspect, Mutex<T> is a smart pointer. More accurately, the call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap. The MutexGuard smart pointer implements Deref to point at our inner data; the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope

    // **** Sharing a Mutex<T> Between Multiple Threads

    // Not thread-safe
    //let counter = Rc::new(Mutex::new(0));

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // numm will get out of scope and release the lock
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Notice, deadlocks can still occur!
}

// Allowing Transference of Ownership Between Threads with Send
pub fn sync() {
    // The Send marker trait indicates that ownership of the type implementing Send can be transferred between threads. Almost every Rust type is Send, but there are some exceptions, including Rc<T>: this cannot be Send because if you cloned an Rc<T> value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time. For this reason, Rc<T> is implemented for use in single-threaded situations where you don’t want to pay the thread-safe performance penalty.
    //
    // Any type composed entirely of Send types is automatically marked as Send as well.

    // **** Allowing Access from Multiple Threads with Sync
    //
    // The smart pointer Mutex<T> is Sync and can be used to share access with multiple threads

    // **** Implementing Send and Sync Manually Is Unsafe
    //
    // Unsafe code. More on https://doc.rust-lang.org/stable/nomicon/send-and-sync.html
}
