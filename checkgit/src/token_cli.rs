use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};


#[derive(Parser)]
#[command(name = "checkgit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    pub username: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    SetToken { token: String },
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    token: String,
}

pub fn config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannot find home directory");
    path.push(".checkgit");

    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create config directory");
    }

    path.push("config.toml");
    path
}

pub fn save_token(token: &str) {
    let config = Config {
        token: token.to_string(),
    };

    let toml = toml::to_string(&config).expect("Failed to serialize config");
    fs::write(config_path(), toml).expect("Failed to write config file");

    println!("Token saved successfully.");
}

pub fn load_token() -> Option<String> {
    let path = config_path();

    if !path.exists() {
        return None;
    }

    let content = fs::read_to_string(path).ok()?;
    let config: Config = toml::from_str(&content).ok()?;
    Some(config.token)
}

pub fn print_token_help() {
    println!("\nGitHub token not found.\n");
    println!("Create one at: https://github.com/settings/tokens");
    println!("Scope needed: read:user\n");
    println!("Then run:");
    println!("  checkgit set-token <your_token>\n");
}