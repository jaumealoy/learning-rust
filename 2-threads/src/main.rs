use std::thread;
use std::sync::*;

const MAX: i64 = 100000000;
const THREAD: i8 = 10;


fn main() {
    println!("Hello, world!");

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    let mut counter: i64 = 0;

    let new_counter: Arc<i64> = Arc::new(0);

    for _i in 0..THREAD {
        let counter_ref = new_counter.clone();
        let thread = thread::spawn(|| worker(counter_ref));
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Total is: {}", new_counter)
}


fn worker(counter: Arc<i64>) {
    println!("Hello from another thread");

    for i in 0..(MAX / (THREAD as i64)) {
        // TODO: increment the counter
    }
}