use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    name: String,
    followers: u32,
    following: u32,
    avatar_url: String,
    bio: String,
    login: String,
    id: u32,
    html_url:String,
    followers_url:String,
    following_url:String,
    location:String,
    email:String,
    twitter_username:String,
    public_repos:u32,
}

pub async fn get_user(username: String) -> Result<User,Error> {
    let client = reqwest::Client::builder()
        .user_agent("Rust-Reqwest-Client")
        .build()?;

    let url = format!("https://api.github.com/users/{}", username);
    let client_data = client.get(&url).send().await?.json::<User>().await?;

    Ok(client_data)
}
