use std::thread;
use std::sync::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::boxed::Box;

const MAX: i64 = 10000;
const THREAD: i8 = 10;


fn main() {
    println!("Hello, world!");

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    let mut counter: i64 = 0;

    let new_counter: Arc<i64> = Arc::new(0);

    let my_counter: Arc<Mutex<Box<i64>>> = Arc::new(Mutex::new(Box::new(0)));

    //let value = my_counter.lock().unwrap();
    //println!("Simple is {0}", value);

    let mut tmp = Box::new(0);
    let a = tmp.as_mut();
    *a = 1;
    println!("tmp is {0}", tmp);
   
    for _i in 0..THREAD {
        let my_ref = my_counter.clone();
        let thread = thread::spawn(move || worker(my_ref));
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Total is: {}", new_counter)
}


fn worker(counter: Arc<Mutex<Box<i64>>>) {
    println!("Hello from another thread");

    for i in 0..(MAX / (THREAD as i64)) {
        let my_box = counter.lock().unwrap();
        // TODO: increment the counter

    }

    let value = counter.lock().unwrap();
    println!("Value is {0}", value)

}