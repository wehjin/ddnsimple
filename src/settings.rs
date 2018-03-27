use std::path::PathBuf;
use super::{AppError, Settings};
use yaml_rust::{Yaml, YamlLoader};


pub fn load() -> Result<Settings, AppError> {
    read_settings_string()
        .and_then(|settings| YamlLoader::load_from_str(&settings).map_err(|_| AppError::FailedToParseSettingsYaml(settings.to_owned())))
        .and_then(parse_yaml)
}

fn parse_yaml(docs: Vec<Yaml>) -> Result<Settings, AppError> {
    let doc = &docs[0];
    let mut settings = Settings {
        account_id: 0,
        access_token: String::new(),
        domain: String::new(),
        record: 0,
    };
    if let Some(value) = doc["account_id"].as_i64() {
        settings.account_id = value as u64;
    } else {
        return Err(AppError::InvalidSetting("account_id".to_owned()));
    }
    if let Some(value) = doc["access_token"]["secret"].as_str() {
        settings.access_token = value.trim().to_owned();
    } else {
        return Err(AppError::InvalidSetting("access_token:secret".to_owned()));
    }
    if let Some(value) = doc["domain"].as_str() {
        settings.domain = value.trim().to_owned();
    } else {
        return Err(AppError::InvalidSetting("domain".to_owned()));
    }
    if let Some(value) = doc["record"].as_i64() {
        settings.record = value as u64;
    } else {
        return Err(AppError::InvalidSetting("record".to_owned()));
    }
    Ok(settings)
}

fn read_settings_string() -> Result<String, AppError> {
    use std::fs::File;
    use std::io::prelude::*;
    let configuration_path = find_settings_path();
    let mut file = File::open(&configuration_path).map_err(|_| AppError::NoSettingsFile(configuration_path.to_owned()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| AppError::FailedToReadSettingsFile(configuration_path.to_owned()))?;
    Ok(contents)
}

fn find_settings_path() -> PathBuf {
    use std::env;
    match env::var("DDNSIMPLE_SETTINGSFILE") {
        Ok(string) => PathBuf::from(string.as_str()),
        Err(_) => {
            let folder = env::home_dir().unwrap_or(PathBuf::from("."));
            folder.join(".ddnsimple.yaml")
        }
    }
}
