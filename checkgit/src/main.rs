use checkgit_core::get_user_profile;

mod helper;
mod token_cli;
mod render;

use clap::Parser;
use token_cli::*;
use render::*;


#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(Commands::SetToken { token }) = cli.command {
        save_token(&token);
        return;
    }

    let username = cli.username.unwrap_or_else(|| {
        println!("Please provide username.");
        std::process::exit(1);
    });

    let token = load_token().or_else(|| std::env::var("GITHUB_TOKEN").ok());

    if token.is_none() {
        print_token_help();
        std::process::exit(1);
    }

    match get_user_profile(&username, token).await {
        Ok(profile) => render(&profile),
        Err(e) => eprintln!("Error: {}", e),
    }
}