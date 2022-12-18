use std::{
    collections::HashMap, // used for storing command line arguments
    env,
};

// Check if the number of command line arguments is correct and if the arguments are valid
pub fn check_args(args: HashMap<String, String>) -> HashMap<String, String> {
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
        //check if the argument is a valid hex string
        if !is_hex(args.get("p").unwrap()) {
            println!("Invalid prefix: {}\nshould be only hex characters", args.get("p").unwrap());
            std::process::exit(1);
        }
        params.insert("p".to_string(), args.get("p").unwrap().to_string());
    } else if args.contains_key("prefix") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("prefix").unwrap()) {
            println!("Invalid prefix: {}\nshould be only hex characters", args.get("prefix").unwrap());
            std::process::exit(1);
        }
        params.insert("p".to_string(), args.get("prefix").unwrap().to_string());
    }
    //if args contains "s" or "suffix" then add it to params
    if args.contains_key("s") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("s").unwrap()) {
            println!("Invalid suffix: {}\nshould be only hex characters", args.get("s").unwrap());
            std::process::exit(1);
        }
        params.insert("s".to_string(), args.get("s").unwrap().to_string());
    } else if args.contains_key("suffix") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("suffix").unwrap()) {
            println!("Invalid suffix: {}\nshould be only hex characters", args.get("suffix").unwrap());
            std::process::exit(1);
        }
        params.insert("s".to_string(), args.get("suffix").unwrap().to_string());
    }
    //if args contains "a" or "anywhere" then add it to params
    if args.contains_key("a") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("a").unwrap()) {
            println!("Invalid anywhere: {}\nshould be only hex characters", args.get("a").unwrap());
            std::process::exit(1);
        }
        params.insert("a".to_string(), args.get("a").unwrap().to_string());
    } else if args.contains_key("anywhere") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("anywhere").unwrap()) {
            println!("Invalid anywhere: {}\nshould be only hex characters", args.get("anywhere").unwrap());
            std::process::exit(1);
        }
        params.insert("a".to_string(), args.get("anywhere").unwrap().to_string());
    }
    params
}

// Parse the command line arguments and return them as a HashMap
pub fn get_args() -> HashMap<String, String> {
    let mut args: HashMap<String, String> = HashMap::new();
    for arg in env::args() {
        let arg_split: Vec<&str> = arg.split("=").collect();
        if arg_split.len() == 2 {
            args.insert(arg_split[0].to_string(), arg_split[1].to_string());
        }
    }
    args
}

//check if strings only contains hexadecimal characters
fn is_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(16))
}

