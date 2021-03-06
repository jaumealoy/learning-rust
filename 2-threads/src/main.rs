use std::thread;
use std::sync::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::boxed::Box;
use std::time::{SystemTime, UNIX_EPOCH};

mod matrix;
use matrix::Matrix;

const MAX: i64 = 1000000;
const THREAD: i8 = 10;


fn main() {
    println!("Hello, world!");

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

    let mut counter: i64 = 0;

    let new_counter: Arc<i64> = Arc::new(0);

    let my_counter: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));

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

    println!("Total is: {}", *my_counter.lock().unwrap());

    // Matrix example
    let mut matrix = Matrix::new(3, 3);
    matrix.set(2, 0, 1);
    matrix.set(0, 2, 1);
    println!("{}", matrix);
    
    let identity = Matrix::identity(3);
    println!("{}", identity);

    let matrix_sum = matrix + Matrix::identity(3);
    println!("{}", matrix_sum);

    let matrix_mul = Matrix::identity(3) * matrix_sum;
    println!("{}", matrix_mul);


    let single_start = SystemTime::now();
    let matrix_mul = Matrix::identity(500) * Matrix::identity(500);

    let single_diff = single_start.elapsed().unwrap();
    println!("{} ms", single_diff.as_millis());

    let thread_start = SystemTime::now();
    let matrix_mul_thread = Matrix::mul(&Matrix::identity(500), &Matrix::identity(500));
    let thread_diff = thread_start.elapsed().unwrap();
    println!("{} ms", thread_diff.as_millis());
}


fn worker(counter: Arc<Mutex<i64>>) {
    println!("Hello from another thread");

    for i in 0..(MAX / (THREAD as i64)) {
        let mut my_box = counter.lock().unwrap();
        // TODO: increment the counter
        *my_box += 1;
    }

    let value = counter.lock().unwrap();
    println!("Value is {0}", *value)

}