mod error;
mod github;
mod models;

use error::*;
use github::*;

#[derive(Debug)]
pub struct UserProfile {
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_image: image::DynamicImage, 
    pub followers: u32,
    pub following: u32,
    pub repo_count: u32,
    pub total_stars: u32,
    pub top_repos: Vec<(String, u32)>,
    pub contribution_matrix: Vec<Vec<u32>>,
}

pub async fn get_user_profile(
    username: &str,
    token: Option<String>,
) -> Result<UserProfile, CheckGitError> {
    let github = GithubClient::new(token)?;

    let user = github.fetch_user(username).await?;

    let (repos, contributions, avatar_image) = tokio::try_join!(
        github.fetch_repos(username),
        github.fetch_contributions(username),
        github.fetch_avatar_image(&user.avatar_url),
    )?;

    let total_stars = calculate_total_stars(&repos);

    let mut sorted = repos.clone();
    sorted.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));

    let top_repos = sorted
        .into_iter()
        .take(3)
        .map(|r| (r.name, r.stargazers_count))
        .collect();

    Ok(UserProfile {
        username: user.login,
        display_name: user.name,
        bio: user.bio,
        avatar_image,
        followers: user.followers,
        following: user.following,
        repo_count: user.public_repos,
        total_stars,
        top_repos,
        contribution_matrix: contributions,
    })
}