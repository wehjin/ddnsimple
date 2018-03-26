use std::path::PathBuf;

fn main() {
    let configuration_path = load_configuration_path();
    println!("Configuration path: {:?}", configuration_path);
}

fn load_configuration_path() -> PathBuf {
    use std::env;
    let folder = env::home_dir().unwrap_or(PathBuf::from("."));
    folder.join(".ddnsimple.yaml")
}
