use std::time::Instant; // used for measuring elapsed time
use std::{
    thread::spawn, // used for spawning threads
    sync::{Arc, Mutex}, // used to create a flag in shared memory to signal to the threads that they should terminate
};
use log; // used for logging messages
use std::sync::mpsc::channel; // used for sending messages between threads

mod args; // used for processing command line arguments
mod logging; // used for logging addresses to a file
mod wallet_generator; // used for generating wallets

fn main() {
    // Create a flag in shared memory to signal to the threads that they should terminate
    let stop_flag = Arc::new(Mutex::new(false));
    let start = Instant::now(); // start the timer

    logging::init(); // Initialize the logger (log to file and console)

    // Process command line arguments and get the appropriate number of threads and wallets to generate
    let params = args::check_args(args::get_args());

    // Set up a channel for sending messages between threads
    let (tx, rx) = channel();

    // Spawn multiple threads
    for _ in 0..params.get("t").unwrap().parse::<u32>().unwrap() {
        let tx_ = tx.clone();
        let params_ = params.clone();
        let stop_flag_ = stop_flag.clone();
        let _ = spawn(move || wallet_generator::thread_function(params_, tx_, stop_flag_));
    }

    log::warn!("Generating {} wallets with {} threads\n\n", params.get("n").unwrap(), params.get("t").unwrap());

    // Wait for wallets to be received on the channel and log them, then stop when reaching the limit
    for _ in 0..params.get("n").unwrap().parse::<u32>().unwrap() {
        let wallet: (String, String) = rx.recv().unwrap();
        logging::log_address(wallet);
    }

    //stop the threads
    *stop_flag.lock().unwrap() = true;

    // Print the total number of wallets generated and the elapsed time
    log::warn!(
        "{} wallets generated in {:?}",
        params.get("n").unwrap(),
        start.elapsed()
    );
}

