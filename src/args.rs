use std::{
    collections::HashMap, // used for storing command line arguments
    env,
};


#[derive(Clone)]
pub struct Parameters {
    pub t: u32,
    pub n: u32,
    pub p: Option<String>,
    pub s: Option<String>,
    pub a: Option<String>,
    pub c: Option<String>,
    pub cn: Option<usize>,
}

// Check if the number of command line arguments is correct and if the arguments are valid
pub fn check_args(args: HashMap<String, String>) -> Parameters {
    let mut params = Parameters {
        t: 1,
        n: 1,
        p: None,
        s: None,
        a: None,
        c: None,
        cn: None,
    };
    // Check if args contains "t" or "threads"
    if !args.contains_key("t") && !args.contains_key("threads") {
        params.t = 1; // default to 1 thread
    } else {
        let t_str = if args.contains_key("t") {
            args.get("t").unwrap()
        } else {
            args.get("threads").unwrap()
        };
        params.t = t_str.parse().expect("Failed to parse thread count");
    }
    // Check if args contains "n" or "number"
    if !args.contains_key("n") && !args.contains_key("number") {
        params.n = 1; // default to generating 1 wallet
    } else {
        let n_str = if args.contains_key("n") {
            args.get("n").unwrap()
        } else {
            args.get("number").unwrap()
        };
        params.n = n_str.parse().expect("Failed to parse number of wallets");
    }
    //if args contains "p" or "prefix" then add it to params
    if args.contains_key("p") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("p").unwrap()) {
            println!("Invalid prefix: {}\nshould be only hex characters", args.get("p").unwrap());
            std::process::exit(1);
        }
        params.p = Some(args.get("p").unwrap().to_string());
    } else if args.contains_key("prefix") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("prefix").unwrap()) {
            println!("Invalid prefix: {}\nshould be only hex characters", args.get("prefix").unwrap());
            std::process::exit(1);
        }
        params.p = Some(args.get("prefix").unwrap().to_string());
    }
    //if args contains "s" or "suffix" then add it to params
    if args.contains_key("s") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("s").unwrap()) {
            println!("Invalid suffix: {}\nshould be only hex characters", args.get("s").unwrap());
            std::process::exit(1);
        }
        params.s = Some(args.get("s").unwrap().to_string());
    } else if args.contains_key("suffix") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("suffix").unwrap()) {
            println!("Invalid suffix: {}\nshould be only hex characters", args.get("suffix").unwrap());
            std::process::exit(1);
        }
        params.s = Some(args.get("suffix").unwrap().to_string());
    }
    //if args contains "a" or "anywhere" then add it to params
    if args.contains_key("a") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("a").unwrap()) {
            println!("Invalid string: {}\nshould be only hex characters", args.get("a").unwrap());
            std::process::exit(1);
        }
        params.a = Some(args.get("a").unwrap().to_string());
    } else if args.contains_key("suffix") {
        //check if the argument is a valid hex string
        if !is_hex(args.get("anywhere").unwrap()) {
            println!("Invalid string: {}\nshould be only hex characters", args.get("anywhere").unwrap());
            std::process::exit(1);
        }
        params.a = Some(args.get("anywhere").unwrap().to_string());
    }

    //if args contains ("c" or "character") and ("cn" or "charNum") then add it to params
    if args.contains_key("c") && args.contains_key("cn") {
        params.c = Some(args.get("c").unwrap().to_string());
        params.cn = Some(args.get("cn").unwrap().parse().expect("Failed to parse character number"));
    } else if args.contains_key("character") && args.contains_key("charNum") {
        params.c = Some(args.get("character").unwrap().to_string());
        params.cn = Some(args.get("charNum").unwrap().parse().expect("Failed to parse character number"));
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