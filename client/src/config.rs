// Constants for configuration settings
pub const CONFIG_FILE: &str = "config.json"; // Path to the config file
pub const CONFIG_BUFFER_SIZE: usize = 1024; // Size of the config value buffer

// Struct to hold configuration settings
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub effects_enabled: bool,
    pub costumes_enabled: bool,
}

// Function to save configuration settings
pub fn save_config(config: &Config) -> Result<(), std::io::Error> {
    let config_str = serde_json::to_string_pretty(config)?;
    std::fs::write(CONFIG_FILE, config_str)
}

// Function to load configuration settings
pub fn load_config() -> Result<Config, serde_json::Error> {
    let config_str = std::fs::read_to_string(CONFIG_FILE)?;
    serde_json::from_str(&config_str)
}