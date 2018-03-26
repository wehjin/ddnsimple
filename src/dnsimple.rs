use reqwest::{Client, Response};
use reqwest::header::*;
use std::error::Error;
use super::{AppError, Settings};

pub fn update(settings: &Settings, ip: &str) -> Result<String, AppError> {
    send_request(settings, ip)
        .and_then(|response| {
            let status = response.status();
            if status.is_success() {
                read_response_text(response)
            } else {
                Err(AppError::UpdateFailed(format!("{}", status)))
            }
        })
}

fn read_response_text(mut response: Response) -> Result<String, AppError> {
    response.text().map_err(|e| AppError::UpdateFailed(e.description().to_owned()))
}

fn send_request(settings: &Settings, ip: &str) -> Result<Response, AppError> {
    let json = format!(r#"{{ "content": "{}"}}"#, ip);
    let location = format!("https://api.dnsimple.com/v2/{}/zones/{}/records/{}", settings.account_id, settings.domain, settings.record);
    Client::new().patch(&location)
                 .body(json)
                 .header(Authorization(Bearer { token: settings.access_token.to_owned() }))
                 .header(ContentType::json())
                 .header(Accept::json())
                 .send()
                 .map_err(|e| { AppError::UpdateFailed(e.description().to_owned()) })
}
