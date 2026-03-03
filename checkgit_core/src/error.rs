use std::fmt;

#[derive(Debug)]
pub enum CheckGitError {
    Network(reqwest::Error),
    UserNotFound,
    RateLimited,
    Unauthorized,
    GithubServerError,
    ImageError(String),
    InvalidResponse,
}

impl fmt::Display for CheckGitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckGitError::Network(e) => write!(f, "Network error: {}", e),
            CheckGitError::UserNotFound => write!(f, "GitHub user not found"),
            CheckGitError::RateLimited => {
                write!(f, "Rate limited. Add GITHUB_TOKEN for higher limits.")
            }
            CheckGitError::Unauthorized => write!(f, "Unauthorized. Invalid token."),
            CheckGitError::GithubServerError => write!(f, "GitHub server error."),
            CheckGitError::ImageError(e) => write!(f, "Image processing error: {}", e),
            CheckGitError::InvalidResponse => write!(f, "Invalid API response."),
        }
    }
}

impl std::error::Error for CheckGitError {}

impl From<reqwest::Error> for CheckGitError {
    fn from(err: reqwest::Error) -> Self {
        CheckGitError::Network(err)
    }
}
