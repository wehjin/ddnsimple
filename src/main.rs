extern crate regex;
extern crate reqwest;

use std::path::PathBuf;

mod current_ip;

#[derive(Debug)]
pub enum AppError {
    NoResponse(String),
    MissingResponseText(String),
    InvalidIpAddress(String),
}

fn main() {
    let configuration_path = load_configuration_path();
    println!("Configuration path: {:?}", configuration_path);

    let current_ip = current_ip::fetch();
    match current_ip {
        Ok(ip) => println!("Ip: {}", ip),
        Err(error) => println!("Error: {:?}", error),
    }
}


fn load_configuration_path() -> PathBuf {
    use std::env;
    let folder = env::home_dir().unwrap_or(PathBuf::from("."));
    folder.join(".ddnsimple.yaml")
}
