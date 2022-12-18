use chrono; // used for getting the current date and time
use log;
use simplelog::{ // used for setting up logging to both a file and the terminal
    CombinedLogger, 
    TermLogger, 
    WriteLogger, 
    LevelFilter, 
    Config, 
    TerminalMode, 
    ColorChoice
};
use std::{
    fs::File, // used for creating a log file
};

pub fn log_address(wallet: (String, String)) {
    log::warn!(
        "----------- new address found -----------\nAddress:     0x{}\nPrivate key: 0x{}\n\n",
        wallet.0,
        wallet.1
    );
}

pub fn init() {
    // Set up logging to both a file and the terminal
    CombinedLogger::init(vec![
        WriteLogger::new(LevelFilter::Warn, Config::default(), File::create(get_date_hour()).unwrap()),
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
    ]).unwrap();
}

// Get the current date and hour in a specific format to use as the log file name
fn get_date_hour() -> String {
    let now = chrono::Local::now();
    let date = now.format("%Y-%m-%d");
    let hour = now.format("%H");
    format!("{}-{}.log", date, hour)
}