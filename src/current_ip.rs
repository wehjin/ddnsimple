use regex::Regex;
use reqwest;
use reqwest::Response;
use std::error::Error;
use super::AppError;

pub fn fetch() -> Result<String, AppError> {
    fetch_ip_response()
        .and_then(|response| read_text_from_response(response))
        .and_then(|text| check_ip_in_text(text))
}

fn fetch_ip_response() -> Result<Response, AppError> {
    reqwest::get("https://api.ipify.org").map_err(|error| AppError::NoIpResponse(error.description().to_owned()))
}

fn read_text_from_response(mut response: Response) -> Result<String, AppError> {
    response.text().map_err(|error| AppError::MissingIpResponseText(error.description().to_owned()))
}

fn check_ip_in_text(text: String) -> Result<String, AppError> {
    let text = text.trim();
    let re = Regex::new(r"^\d+[.]\d+[.]\d+[.]\d+$").unwrap();
    if re.is_match(text) {
        Ok(text.to_owned())
    } else {
        Err(AppError::InvalidIpAddress(text.to_owned()))
    }
}
