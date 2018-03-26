extern crate regex;
extern crate reqwest;
extern crate yaml_rust;

use std::path::PathBuf;

mod current_ip;
mod settings;
mod dnsimple;

#[derive(Debug)]
pub enum AppError {
    NoSettingsFile(PathBuf),
    FailedToReadSettingsFile(PathBuf),
    FailedToParseSettingsYaml(String),
    InvalidSetting(String),
    NoResponseFromIpService(String),
    NoTextInIpServiceResponse(String),
    InvalidIpAddress(String),
    UpdateFailed(String),
}

#[derive(Debug)]
pub struct Settings {
    pub account_id: u64,
    pub access_token: String,
    pub domain: String,
    pub record: u64,
}

fn main() {
    match update_record() {
        Ok(success) => println!("Updated: {:?}", success),
        Err(error) => println!("Error: {:?}", error),
    }
    println!("Sleeping: 1 hour");
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn update_record() -> Result<String, AppError> {
    let settings = settings::load()?;
    let current_ip = current_ip::fetch()?;
    dnsimple::update(&settings, &current_ip)
}

