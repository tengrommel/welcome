use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use rand::prelude::*;

fn main() {
    // Spawn 100 tasks on threads
    let thread_handles: Vec<JoinHandle<_>> =
        (1..=100).map(|i|{
            // 1-2 second delay
            let delay = rand::thread_rng().gen_range(1000, 2000);
            // Give this thread a unique name
            let builder = thread::Builder::new().name(format!("custom-{}", i));
            // Spawn it
            builder.spawn(move || {
                // Step 1: delay
                thread::sleep(Duration::from_millis(delay));
                // Step 2: print to screen
                println!("Delay {} ms done! Thread name: {}", delay, thread::current().name().unwrap());
            }).unwrap()
        }).collect();
    // Wait for all of them to finish
    for h in thread_handles {
        let _ = h.join();
    }
    println!("Job done");
}
