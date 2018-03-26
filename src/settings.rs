use std::path::PathBuf;
use super::AppError;

pub fn load() -> Result<String, AppError> {
    use std::fs::File;
    use std::io::prelude::*;
    let configuration_path = find_settings_path();
    let mut file = File::open(&configuration_path).map_err(|_| AppError::NoSettingsFile(configuration_path.to_owned()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| AppError::UnableToReadSettingsFile(configuration_path.to_owned()))?;
    Ok(contents)
}

fn find_settings_path() -> PathBuf {
    use std::env;
    let folder = env::home_dir().unwrap_or(PathBuf::from("."));
    folder.join(".ddnsimple.yaml")
}
