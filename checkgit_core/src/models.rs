use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<GraphQLUser>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLUser {
    pub user: ContributionsCollection,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionsCollection {
    pub contributions_collection: ContributionCalendarWrapper,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionCalendarWrapper {
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

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}