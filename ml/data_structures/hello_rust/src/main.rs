use std::thread;

fn threading() {
    let x = 0;
    let handle = thread::spawn(move ||{
        println!("Hello from a thread, the number is: {}", x);
    });
    handle.join().unwrap();
}

fn main() {
    threading();
}
