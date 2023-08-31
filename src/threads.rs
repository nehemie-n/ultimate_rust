use rand::{thread_rng, Rng};
use std::sync::mpsc::channel;
use std::thread::{self, JoinHandle};
const THREADS_N: i32 = 100;

// Import the env module for accessing command-line arguments
use std::env;
use std::time::Duration;

/// compute number of threads to create
pub fn get_args() -> i32 {
    let args: Vec<String> = env::args().into_iter().collect();
    let mut n = THREADS_N.clone();
    if args.len() > 1 {
        n = args[1].clone().parse::<i32>().unwrap_or(THREADS_N.clone());
        println!("{n:?}");
    }
    return n;
}

/// Define a function that simulates some computation using threads
pub fn _simulate() {
    // Create a mutable vector for storing the thread handles
    let mut threads: Vec<JoinHandle<i32>> = Vec::new();
    let n = get_args();
    // Loop from 0 to n (exclusive)
    for i in 0..n {
        // Spawn a new thread with a closure that takes i as an argument
        let handle = thread::spawn(move || {
            println!("thread {i}");
            return rand::random::<i32>();
        });
        // Push the thread handle to the vector
        threads.push(handle);
    }

    // Loop over the vector of thread handles
    for thread in threads {
        // Wait for the thread to finish and get its return value
        let _ = thread.join().unwrap();
    }
}

///
///
/// Usage of channels for threads communication
pub fn simulate_threads_with_channels() {
    let n = get_args();
    let mut threads: Vec<JoinHandle<i32>> = Vec::new();
    let (sender, receiver) = channel::<i32>();

    for i in 0..n {
        // Spawn a new thread with a closure that takes i as an argument
        let sender = sender.clone();
        let handle = thread::spawn(move || {
            let _ = sender.send(i);
            return rand::random::<i32>();
        });
        // Push the thread handle to the vector
        threads.push(handle);
    }

    let exit_nbr = thread_rng().gen_range(0..n);
    println!("EXIT = {exit_nbr}");

    for value in receiver {
        println!("RECEIVED = {value}");
        if value == exit_nbr {
            break;
        }
    }

    println!("Printed out of receiver")
}
