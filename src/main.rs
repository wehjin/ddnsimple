extern crate chrono;
extern crate clap;
extern crate regex;
extern crate reqwest;
extern crate yaml_rust;

use chrono::Duration;
use chrono::prelude::*;
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

struct State {
    pub count: Count,
    pub rest_duration: Duration,
}

impl Default for State {
    fn default() -> Self {
        State {
            count: Count::Infinity,
            rest_duration: Duration::hours(4),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Count {
    None,
    One,
    Infinity,
}

impl Count {
    fn decrement(&self) -> Self {
        match self {
            &Count::None => Count::None,
            &Count::One => Count::None,
            &Count::Infinity => Count::Infinity,
        }
    }
}

fn main() {
    let mut state = init_state();
    while state.count != Count::None {
        let now = Local::now();
        print_status("AWAKE", &format!("at: {}", now));


        let result = update_record();
        match result {
            Ok(ok) => print_status("UPDATE", &format!("received: {:?}", ok)),
            Err(err) => print_status("ERROR", &format!("failed with: {:?}", err)),
        }

        state.count = state.count.decrement();
        if state.count != Count::None {
            let wake_time = now + state.rest_duration;
            print_status("SLEEP", &format!("wake in: {}s, at: {}", state.rest_duration.num_seconds(), wake_time));
            std::thread::sleep(state.rest_duration.to_std().unwrap());
        }
    }
}

fn init_state() -> State {
    let mut state: State = Default::default();
    use clap::{App, Arg};
    let matches = App::new("ddnsimple")
        .about("Updates a dnsimple.com A record with the host's public ip address")
        .arg(Arg::with_name("once").short("1").long("once").help("Stop after one update"))
        .get_matches();
    if matches.is_present("once") {
        state.count = Count::One;
    }
    state
}

fn print_status(status: &str, detail: &str) {
    const MIN_STATUS_WIDTH: usize = 6;
    let num_spaces = std::cmp::max(0, MIN_STATUS_WIDTH - status.len());
    let spaces = " ".repeat(num_spaces);
    println!("{}{}  {}", &spaces, status, detail);
}

fn update_record() -> Result<String, AppError> {
    let settings = settings::load()?;
    let current_ip = current_ip::fetch()?;
    dnsimple::update(&settings, &current_ip)
}

