extern crate regex;
extern crate reqwest;

use std::path::PathBuf;

mod current_ip;
mod settings;

#[derive(Debug)]
pub enum AppError {
    NoSettingsFile(PathBuf),
    UnableToReadSettingsFile(PathBuf),
    NoIpResponse(String),
    MissingIpResponseText(String),
    InvalidIpAddress(String),
}

#[derive(Debug)]
pub struct Settings {
    pub account_id: String,
    pub access_token: String,
    pub domain: String,
    pub record: u64,
    pub ttl: u64,
}

fn main() {
    let settings = settings::load();
    match settings {
        Ok(configuration) => println!("Configuration: {:?}", configuration),
        Err(error) => println!("Error: {:?}", error),
    }

    let current_ip = current_ip::fetch();
    match current_ip {
        Ok(ip) => println!("Ip: {}", ip),
        Err(error) => println!("Error: {:?}", error),
    }
}

