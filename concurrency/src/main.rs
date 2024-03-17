use std::thread;
use std::time::Duration;

// The goal of this function is to demonstrate concurrency
//
// We first spawn a closure(?) in a thread which will run
// in parallel with the 'main thread'
//
// Watch the output! Notice that the child thread doesn't 
// execute fully because the main thread finishes first
fn main() {
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

     // Uncomment this to allow the first thread to complete!
     // handle.join().unwrap();
}