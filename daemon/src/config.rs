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
    /// Default input mode (pre-edit, surrounding text, etc.).
    pub default_input_mode: i32,
    /// Compilation and processing flags for the engine.
    pub flags: u32,
    /// Mapping of window classes to specific input modes.
    pub input_mode_mapping: HashMap<String, i32>,
    /// Stored active Vietnamese typing layout (Telex or VNI) used when toggling back from English mode.
    pub vietnamese_layout: String,
    /// Output charset encoding (e.g., "Unicode", "TCVN3", "VNI", "VIQR").
    pub charset: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_method: "Telex".to_string(),
            default_input_mode: 1, // PREEDIT_IM
            flags: ESTD_FLAGS,
            input_mode_mapping: HashMap::new(),
            vietnamese_layout: "Telex".to_string(),
            charset: "Unicode".to_string(),
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
