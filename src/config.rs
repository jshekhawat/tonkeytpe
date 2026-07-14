use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub theme: String,
    pub always_on_top: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            always_on_top: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(config_dir) = dirs::config_dir() {
            let path = config_dir.join("tonkeytype").join("config.toml");
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    return toml::from_str(&content).unwrap_or_default();
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(config_dir) = dirs::config_dir() {
            let dir = config_dir.join("tonkeytype");
            let _ = std::fs::create_dir_all(&dir);
            let path = dir.join("config.toml");
            if let Ok(content) = toml::to_string_pretty(self) {
                let _ = std::fs::write(&path, content);
            }
        }
    }
}
