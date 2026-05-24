use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Settings {
    pub dgpu: bool,
    pub wayland: bool,
    pub hide_while_running: bool,
}

impl Settings {
    pub fn load() -> Self {
        let path = get_config_path();
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(settings) = toml::from_str(&content) {
                return settings;
            }
        }
        Settings {
            dgpu: true,
            wayland: false,
            hide_while_running: true,
        }
    }

    pub fn save(&self) {
        let path = get_config_path();
        if let Ok(content) = toml::to_string(self) {
            let _ = fs::write(path, content);
        }
    }
}

fn get_config_path() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".config")
        });
    let app_config_dir = config_dir.join("tetrio-launcher");
    let _ = fs::create_dir_all(&app_config_dir);
    app_config_dir.join("config.toml")
}
