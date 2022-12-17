use std::time::Instant; // used for measuring elapsed time
use std::{
    fs::File, // used for creating a log file
    thread::spawn, // used for spawning threads
    collections::HashMap, // used for storing command line arguments
    env,
};
use rand_chacha::ChaChaRng; // used for generating random numbers
use libsecp256k1::{PublicKey, SecretKey}; // used for generating secret and public keys
use sha3::{Digest, Keccak256}; // used for generating hashes using the Keccak-256 algorithm
use log; // used for logging messages
use std::sync::mpsc::{channel, Sender}; // used for sending messages between threads
use simplelog::*; // used for setting up logging to both a file and the terminal
use chrono; // used for getting the current date and time
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::rand_core::RngCore;


fn main() {
    // Set up logging to both a file and the terminal
    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Warn, Config::default(), File::create(get_date_hour()).unwrap()),
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
    ]).unwrap();

    let start = Instant::now(); // start the timer

    // Process command line arguments and get the appropriate number of threads and wallets to generate
    let params = check_args(get_args());

    // Set up a channel for sending messages between threads
    let (tx, rx) = channel();

    // Spawn multiple threads
    for _ in 0..params.get("t").unwrap().parse::<u32>().unwrap() {
        let tx_ = tx.clone();
        let params_ = params.clone();
        let _ = spawn(move || thread_function(params_, tx_));
    }

    // Wait for wallets to be received on the channel and log them, then stop when reaching the limit
    for _ in 0..params.get("n").unwrap().parse::<u32>().unwrap() {
        let wallet: HashMap<String, String> = rx.recv().unwrap();
        log::warn!(
            "----------- new address found -----------\nAddress:     0x{}\nPrivate key: 0x{}\n\n",
            wallet.get("address").unwrap(),
            wallet.get("private_key").unwrap()
        );
    }

    // Print the total number of wallets generated and the elapsed time
    println!(
        "{} wallets generated in {:?}",
        params.get("n").unwrap(),
        start.elapsed()
    );
}

// Get the current date and hour in a specific format to use as the log file name
fn get_date_hour() -> String {
    let now = chrono::Local::now();
    let date = now.format("%Y-%m-%d");
    let hour = now.format("%H");
    format!("{}-{}.log", date, hour)
}

// Parse the command line arguments and return them as a HashMap
fn get_args() -> HashMap<String, String> {
    let mut args: HashMap<String, String> = HashMap::new();
    for arg in env::args() {
        let arg_split: Vec<&str> = arg.split("=").collect();
        if arg_split.len() == 2 {
            args.insert(arg_split[0].to_string(), arg_split[1].to_string());
        }
    }
    args
}

// Check if the number of command line arguments is correct and if the arguments are valid
fn check_args(args: HashMap<String, String>) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();
    // Check if args contains "t" or "threads"
    if !args.contains_key("t") && !args.contains_key("threads") {
        params.insert("t".to_string(), "1".to_string()); // default to 1 thread
    } else {
        if args.contains_key("t") {
            params.insert("t".to_string(), args.get("t").unwrap().to_string());
        } else {
            params.insert("t".to_string(), args.get("threads").unwrap().to_string());
        }
    }
    // Check if args contains "n" or "number"
    if !args.contains_key("n") && !args.contains_key("number") {
        params.insert("n".to_string(), "1".to_string()); // default to generating 1 wallet
    } else {
        if args.contains_key("n") {
            params.insert("n".to_string(), args.get("n").unwrap().to_string());
        } else {
            params.insert("n".to_string(), args.get("number").unwrap().to_string());
        }
    }
    //if args contains "p" or "prefix" then add it to params
    if args.contains_key("p") {
        params.insert("p".to_string(), args.get("p").unwrap().to_string());
    } else if args.contains_key("prefix") {
        params.insert("p".to_string(), args.get("prefix").unwrap().to_string());
    }
    //if args contains "s" or "suffix" then add it to params
    if args.contains_key("s") {
        params.insert("s".to_string(), args.get("s").unwrap().to_string());
    } else if args.contains_key("suffix") {
        params.insert("s".to_string(), args.get("suffix").unwrap().to_string());
    }
    //if args contains "a" or "anywhere" then add it to params
    if args.contains_key("a") {
        params.insert("a".to_string(), args.get("a").unwrap().to_string());
    } else if args.contains_key("anywhere") {
        params.insert("a".to_string(), args.get("anywhere").unwrap().to_string());
    }
    params
}

//generate a random local wallet
fn generate_wallet() -> HashMap<String, String> {    
    let mut rng = ChaChaRng::from_entropy();
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    // Generate a secret key
    let secret_key = SecretKey::parse(&bytes).unwrap();
    // Generate a public key from the secret key
    let public_key = PublicKey::from_secret_key(&secret_key);
    // Hash the public key to get the Ethereum address
    let public_key_hash = Keccak256::digest(&public_key.serialize()[1..]);
    let address = hex::encode(&public_key_hash[12..]);

    let mut wallet: HashMap<String, String> = HashMap::new();
    wallet.insert("address".to_string(), address.to_string());
    wallet.insert("private_key".to_string(), hex::encode(&secret_key.serialize()).to_string());
    wallet
}


//thread function to generate wallets, check if they comply with params and send it back through channel
fn thread_function(params: HashMap<String, String>, tx: Sender<HashMap<String, String>>) {
    loop {
        let wallet = generate_wallet();
        let address = wallet.get("address").unwrap();
        if !check_address(address, params.clone()) {
            continue;
        }
        tx.send(wallet).unwrap();
    }
}

//check if address complies with params
fn check_address(address: &str, params: HashMap<String, String>) -> bool {
    if params.contains_key("p") {
        if !address.starts_with(params.get("p").unwrap()) {
            return false;
        }
    }
    if params.contains_key("s") {
        if !address.ends_with(params.get("s").unwrap()) {
            return false;
        }
    }
    if params.contains_key("a") {
        if !address.contains(params.get("a").unwrap()) {
            return false;
        }
    }
    true
}