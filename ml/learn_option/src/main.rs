use std::thread;
use std::time::Duration;

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
    // thread: thread::JoinHandle<()>
}

impl Worker {
    fn new() -> Worker {
        let thread = thread::spawn(move ||{
            println!("start work 10 secs...");
            thread::sleep(Duration::from_secs(10));
        });
        Worker {thread: Some(thread)}
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool{
        assert!(size>0);
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new());
        }
        ThreadPool{
            workers
        }
    }
}


impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            // worker.thread.join().unwrap();
            // println!("worker thread finished!")
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            }
        }
    }
}

fn main() {

}
