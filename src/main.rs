use std::{fs::File, thread::spawn, collections::HashMap, env};
use rand::Rng;
use libsecp256k1::{PublicKey, SecretKey};
use sha3::{Digest, Keccak256};
use log;
use std::sync::mpsc::{channel, Sender};
use simplelog::*;
use chrono;

fn main() {
    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Warn, Config::default(), File::create(get_date_hour()).unwrap()),
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
    ]).unwrap();
    let params = check_args(get_args());
    let (tx, rx) = channel();


    for _ in 0..params.get("t").unwrap().parse::<u32>().unwrap() {
        let tx_ = tx.clone();
        let params_ = params.clone();
        let _ = spawn(move || thread_function(params_, tx_));
    }

    //wait for wallets in channel and log them, then stop when reaching the limit
    for _ in 0..params.get("n").unwrap().parse::<u32>().unwrap() {
        let wallet: HashMap<String, String> = rx.recv().unwrap();
        log::warn!("----------- new address found -----------\nAddress:     0x{}\nPrivate key: 0x{}\n\n", wallet.get("address").unwrap(), wallet.get("private_key").unwrap());
    }


}

//get date and hour to use as log file name
fn get_date_hour() -> String {
    let now = chrono::Local::now();
    let date = now.format("%Y-%m-%d");
    let hour = now.format("%H");
    format!("{}-{}.log", date, hour)
}




//get hashmap of command line args starting with "-"
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

//check if number of args is correct and if the args are valid
fn check_args(args: HashMap<String, String>) -> HashMap<String, String> {
    let mut params: HashMap<String, String> = HashMap::new();
    //check if args contains "t" or "threads"
    if !args.contains_key("t") && !args.contains_key("threads") {
        params.insert("t".to_string(), "1".to_string());
    } else {
        if args.contains_key("t") {
            params.insert("t".to_string(), args.get("t").unwrap().to_string());
        } else {
            params.insert("t".to_string(), args.get("threads").unwrap().to_string());
        }
    }
    //check if args contains "n" or "number"
    if !args.contains_key("n") && !args.contains_key("number") {
        params.insert("n".to_string(), "1".to_string());
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
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();

    let private_key = SecretKey::parse(&random_bytes).unwrap();
    let public_key = PublicKey::from_secret_key(&private_key);

    let public_key_hash = Keccak256::digest(&public_key.serialize()[1..]);
    let address = hex::encode(&public_key_hash[12..]);

    let mut wallet: HashMap<String, String> = HashMap::new();
    wallet.insert("address".to_string(), address.to_string());
    wallet.insert("private_key".to_string(), hex::encode(&private_key.serialize()).to_string());
    wallet
}


//thread function to generate wallets, check if they comply with params and send it back through channel
fn thread_function(params: HashMap<String, String>, tx: Sender<HashMap<String, String>>) {
    loop {
        let wallet = generate_wallet();
        let address = wallet.get("address").unwrap();

        if params.contains_key("p") {
            if !address.starts_with(params.get("p").unwrap()) {
                continue;
            }
        }
        if params.contains_key("s") {
            if !address.ends_with(params.get("s").unwrap()) {
                continue;
            }
        }
        if params.contains_key("a") {
            if !address.contains(params.get("a").unwrap()) {
                continue;
            }
        }
        tx.send(wallet).unwrap();
    }
}