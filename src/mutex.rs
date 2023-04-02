// listing all prime numbers less than specified number using desired number of threads

use std::sync::{Arc, Mutex};
use std::thread;

// adjust the number of threads spawned using this constant
const THREAD_COUNT: u32 = 3;
const UPPER_LIMIT: u32 = 10000;

fn is_prime(num: u32) -> bool {
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    if num <= 1 {
        false
    } else {
        true
    }
}

fn main() {
    assert!(THREAD_COUNT > 0, "Atleast one threads should be present");
    let mut primes = Arc::new(Mutex::new(Vec::new()));
    let elements_per_thread = UPPER_LIMIT / THREAD_COUNT;
    let mut handles = Vec::new();
    for i in 0..THREAD_COUNT - 1 {
        let list_clone = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            for num in i * elements_per_thread..i * elements_per_thread + elements_per_thread {
                if is_prime(num) {
                    let mut lock = list_clone.lock().unwrap();
                    lock.push(num);
                }
            }
        });
        handles.push(handle);
    }
    let list_clone = Arc::clone(&primes);
    thread::spawn(move || {
        for num in (THREAD_COUNT - 1) * elements_per_thread..UPPER_LIMIT {
            if is_prime(num) {
                let mut lock = list_clone.lock().unwrap();
                lock.push(num);
            }
        }
    })
    .join()
    .unwrap();
    for handle in handles {
        handle.join().unwrap();
    }
    let lock = primes.lock().unwrap();
    println!("Prime numbers less than {UPPER_LIMIT} = {}", lock.len());
    println!("{:?}", lock);
}
