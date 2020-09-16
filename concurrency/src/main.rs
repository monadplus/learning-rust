use std::thread;
use std::time::Duration;

mod lib;

// **** Fearless Concurrency ****
fn main() {
    // The green-threading M:N model requires a larger language runtime to manage threads. As such, the Rust standard library only provides an implementation of 1:1 threading. Because Rust is such a low-level language, there are crates that implement M:N threading if you would rather trade overhead for aspects such as more control over which threads run when and lower costs of context switching, for example.

    // **** Creating New Thread with spawn ****

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // **** Using `move` Closures with Threads

    let v = vec![1, 2, 3];

    // There’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid. We could call drop(v) while the thread is executing;
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // **** Using Message Passing to Transfer Data Between Threads ****
    lib::message_passing()
}
