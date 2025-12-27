use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use colored::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub user_name: Option<String>,
    pub theme: Option<String>,
}

pub fn get_config_dir() -> Result<PathBuf> {
    let mut path = dirs::config_dir().context("Could not find config directory")?;
    path.push("neo");
    Ok(path)
}

pub fn load_config() -> Result<Config> {
    let mut path = get_config_dir()?;
    path.push("config.toml");

    if !path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(&path).context("Failed to read config file")?;
    let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
    Ok(config)
}

pub fn init_config() -> Result<()> {
    let path = get_config_dir()?;
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    
    let file_path = path.join("config.toml");
    if file_path.exists() {
        println!("{}", "Config file already exists.".yellow());
        return Ok(());
    }

    let default_config = Config {
        user_name: Some("User".to_string()),
        theme: Some("default".to_string()),
    };

    let toml_string = toml::to_string_pretty(&default_config)?;
    fs::write(&file_path, toml_string)?;

    println!("{} Configuration initialized at {:?}", "✔".green(), file_path);
    Ok(())
}

pub fn show_config_location() -> Result<()> {
    let mut path = get_config_dir()?;
    path.push("config.toml");
    println!("Config file located at: {}", path.display().to_string().blue());
    Ok(())
}
