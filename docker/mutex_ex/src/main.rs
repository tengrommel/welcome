use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

fn main() {
    // Rc<T>不是线程安全的
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("res = {}", *counter.lock().unwrap());
    println!("end")
}
