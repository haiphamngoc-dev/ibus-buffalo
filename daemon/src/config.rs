use crate::utils::get_user_config_dir;
use buffalo_core::ESTD_FLAGS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

/// Stored active configuration structure for the IBus Buffalo input method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Active input method name (e.g. "Telex", "VNI", "English").
    pub input_method: String,
    /// Compilation and processing flags for the engine.
    pub flags: u32,
    /// Stored active Vietnamese typing layout (Telex or VNI) used when toggling back from English mode.
    pub vietnamese_layout: String,
    /// Output charset encoding (e.g., "Unicode", "TCVN3", "VNI", "VIQR").
    pub charset: String,
    /// Toggle to enable or disable shorthand (macro) replacements.
    pub enable_macro: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_method: "Telex".to_string(),
            flags: ESTD_FLAGS,
            vietnamese_layout: "Telex".to_string(),
            charset: "Unicode".to_string(),
            enable_macro: false,
        }
    }
}

/// Loads configuration from `$XDG_CONFIG_HOME/ibus-buffalo/config.toml`.
/// Fallbacks to `Config::default()` and saves it if the file does not exist.
pub fn load_config() -> Config {
    let path = get_user_config_dir()
        .join("ibus-buffalo")
        .join("config.toml");
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(cfg) = toml::from_str(&content) {
            return cfg;
        }
    }
    let cfg = Config::default();
    let _ = save_config(&cfg);
    cfg
}

/// Saves configuration to `$XDG_CONFIG_HOME/ibus-buffalo/config.toml`.
pub fn save_config(cfg: &Config) -> std::io::Result<()> {
    let dir = get_user_config_dir().join("ibus-buffalo");
    fs::create_dir_all(&dir)?;
    let path = dir.join("config.toml");
    let content = toml::to_string_pretty(cfg).unwrap();
    fs::write(path, content)?;
    Ok(())
}

/// Loads shorthand replacements from `$XDG_CONFIG_HOME/ibus-buffalo/macro.txt`.
/// Lines starting with '#' or empty lines are ignored.
/// Format is `shorthand:replacement_text`.
pub fn load_macro_table() -> HashMap<String, String> {
    let mut table = HashMap::new();
    let path = get_user_config_dir().join("ibus-buffalo").join("macro.txt");
    if let Ok(content) = fs::read_to_string(&path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_string();
                let val = line[pos + 1..].trim().to_string();
                if !key.is_empty() && !val.is_empty() {
                    table.insert(key, val);
                }
            }
        }
    }
    table
}
