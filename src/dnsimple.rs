use reqwest::{Client, Response};
use reqwest::header::*;
use std::error::Error;
use super::{AppError, Settings};
use serde_json;

pub fn update_record_content(settings: &Settings, ip: &str) -> Result<String, AppError> {
    send_patch(settings, ip)
        .and_then(|response| {
            let status = response.status();
            if status.is_success() {
                read_response_text(response)
            } else {
                Err(AppError::UpdateRecordFailed(format!("{}", status)))
            }
        })
}

pub fn read_record_content(settings: &Settings) -> Result<String, AppError> {
    send_get(settings)
        .and_then(|mut response| {
            let status = response.status();
            if status.is_success() {
                response.text().map_err(|e| AppError::ReadRecordFailed(e.description().to_owned()))
                    .and_then(|text| {
                        serde_json::from_str(&text).map_err(|e| AppError::ReadRecordFailed(e.description().to_owned()))
                            .map(|value: serde_json::Value| {
                                value["data"]["content"].as_str().unwrap_or(text.as_str()).to_owned()
                            })
                    })
            } else {
                Err(AppError::ReadRecordFailed(format!("{}", status)))
            }
        })
}

fn read_response_text(mut response: Response) -> Result<String, AppError> {
    response.text().map_err(|e| AppError::UpdateRecordFailed(e.description().to_owned()))
}

fn send_patch(settings: &Settings, ip: &str) -> Result<Response, AppError> {
    let json = format!(r#"{{ "content": "{}"}}"#, ip);
    let location = format!("https://api.dnsimple.com/v2/{}/zones/{}/records/{}", settings.account_id, settings.domain, settings.record);
    Client::new().patch(&location)
        .body(json)
        .header(Authorization(Bearer { token: settings.access_token.to_owned() }))
        .header(ContentType::json())
        .header(Accept::json())
        .send()
        .map_err(|e| { AppError::UpdateRecordFailed(e.description().to_owned()) })
}

fn send_get(settings: &Settings) -> Result<Response, AppError> {
    let location = format!("https://api.dnsimple.com/v2/{}/zones/{}/records/{}", settings.account_id, settings.domain, settings.record);
    Client::new().get(&location)
        .header(Authorization(Bearer { token: settings.access_token.to_owned() }))
        .header(ContentType::json())
        .header(Accept::json())
        .send()
        .map_err(|e| { AppError::UpdateRecordFailed(e.description().to_owned()) })
}
