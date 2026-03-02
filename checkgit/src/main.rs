use checkgit_core::get_user_profile;

mod token_cli;

use clap::Parser;
use colored::*;
use token_cli::*;
use viuer::{print, Config};

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
        Ok(profile) => render(profile),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn render(profile: checkgit_core::UserProfile) {
    print(
        &profile.avatar_image,
        &Config {
            width: Some(40),
            ..Default::default()
        },
    )
    .unwrap();

    println!();

    let name = profile
        .display_name
        .clone()
        .unwrap_or(profile.username.clone());

    println!("{}", name.bold().bright_white());
    println!("{}", format!("@{}", profile.username).bright_black());

    println!();
    println!("{}  {}", "Followers:".bright_blue(), profile.followers);
    println!("{}  {}", "Following:".bright_blue(), profile.following);
    println!("{}  {}", "Repos:".bright_blue(), profile.repo_count);
    println!("{}  {}", "Stars:".bright_blue(), profile.total_stars);

    println!();
    println!("{}", "Top Repositories".bold().bright_white());

    for (name, stars) in profile.top_repos {
        println!("  ★ {:<20} {}", name, stars.to_string().yellow());
    }

    println!();
    println!("{}", "Contribution Heatmap".bold().bright_white());
    println!();

    render_heatmap(profile.contribution_matrix);
}

fn render_heatmap(matrix: Vec<Vec<u32>>) {
    for row in matrix {
        for value in row {
            let block = match value {
                0 => "  ".on_bright_black(),
                1..=2 => "  ".on_green(),
                3..=5 => "  ".on_bright_green(),
                6..=10 => "  ".on_truecolor(0, 255, 0),
                _ => "  ".on_truecolor(0, 200, 0),
            };
            print!("{}", block);
        }
        println!();
    }
}