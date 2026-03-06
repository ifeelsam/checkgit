use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<UserData>,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub user: User,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub contributions_collection: ContributionsCollection,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionsCollection {
    pub total_commit_contributions: u32,
    pub total_issue_contributions: u32,
    pub total_pull_request_contributions: u32,
    pub total_pull_request_review_contributions: u32,
    pub total_repositories_with_contributed_commits: u32,

    pub contribution_calendar: ContributionCalendar,
}

#[derive(Debug, Deserialize)]
pub struct ContributionCalendar {
    pub weeks: Vec<Week>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionDay {
    pub contribution_count: u32,
}

#[derive(Debug)]
pub struct ContributionStats {
    pub commits: u32,
    pub pull_requests: u32,
    pub reviews: u32,
    pub issues: u32,
    pub repos_contributed: u32,
}