use leptos::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::MySqlPool;
use std::time::Duration;
use time::{
    format_description, macros::format_description, Date,
};
use crate::Username;

#[cfg(feature = "ssr")]
pub fn pool() -> Result<MySqlPool, ServerFnError> {
    use_context::<MySqlPool>().ok_or_else(|| {
        ServerFnError::ServerError("Pool missing.".into())
    })
}

#[cfg(feature = "ssr")]
pub fn with_admin_access() -> Result<Username, ServerFnError> {
    let Some(Username(username)) = use_context::<Option<crate::Username>>().flatten() else {
        return Err(ServerFnError::ServerError("not allowed to submit via admin panel".to_string()));
    };
    if username != "ChristopherBiscardi" {
        return Err(ServerFnError::ServerError("not allowed to submit via admin panel".to_string()));
    }
    Ok(Username(username))
}

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlPullRequestInfo {
    github_id: String,
    url: String,
    title: String,
    author: String,
    author_url: String,
    labels: serde_json::Value,
    merged_at_date: Option<time::Date>,
}

#[derive(Debug, Deserialize)]
struct LabelInfo {
    name: String,
    color: String,
}


#[server]
pub async fn get_merged_pull_requests(
    date: time::Date,
) -> Result<Vec<ClientPullRequestInfo>, ServerFnError> {
    let pool = pool()?;

    // one week
    let start_date =
        date - Duration::from_secs(60 * 60 * 24 * 7);

    let results: Vec<SqlPullRequestInfo> = sqlx::query_as!(
        SqlPullRequestInfo,
        "SELECT
        github_id,
        url,
        title,
        author,
        author_url,
        labels,
        merged_at_date
FROM merged_pull_request
WHERE merged_at_date BETWEEN ? AND ?
ORDER BY merged_at_date DESC",
        Some(start_date),
        Some(date),
    )
    .fetch_all(&pool)
    .await?;

    // dbg!(results);
    Ok(results
        .into_iter()
        .map(|v| ClientPullRequestInfo::from(v))
        .collect())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientPullRequestInfo {
    pub github_id: String,
    pub url: String,
    pub title: String,
    pub author: String,
    pub author_url: String,
    pub labels: serde_json::Value,
    pub merged_at_date: String,
}

#[cfg(feature = "ssr")]
impl From<SqlPullRequestInfo> for ClientPullRequestInfo {
    fn from(value: SqlPullRequestInfo) -> Self {

        ClientPullRequestInfo {
            github_id: value.github_id,
            url: value.url,
            title: value.title,
            author: value.author,
            author_url: value.author_url,
            labels: value.labels,
            merged_at_date: value
                .merged_at_date
                .expect("a date, because the query was for valid date ranges")
                .format(&crate::issue_date::issue_date_format)
                .expect("a valid format"),
        }
    }
}

// OpenedPullRequests

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlOpenedPullRequestInfo {
    github_id: String,
    url: String,
    title: String,
    author: String,
    author_url: String,
    labels: serde_json::Value,
    gh_created_at: Option<time::Date>,
}

#[server]
pub async fn get_opened_pull_requests(
    date: time::Date,
) -> Result<Vec<ClientOpenedPullRequestInfo>, ServerFnError>
{
    let pool = pool()?;

    // one week
    let start_date =
        date - Duration::from_secs(60 * 60 * 24 * 7);

    let results: Vec<SqlOpenedPullRequestInfo> =
        sqlx::query_as!(
            SqlOpenedPullRequestInfo,
            "SELECT
        github_id,
        url,
        title,
        author,
        author_url,
        labels,
        gh_created_at
FROM new_pull_request
WHERE gh_created_at BETWEEN ? AND ?
ORDER BY gh_created_at DESC",
            Some(start_date),
            Some(date),
        )
        .fetch_all(&pool)
        .await?;

    Ok(results
        .into_iter()
        .map(|v| ClientOpenedPullRequestInfo::from(v))
        .collect())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOpenedPullRequestInfo {
    pub github_id: String,
    pub url: String,
    pub title: String,
    pub author: String,
    pub author_url: String,
    pub labels: serde_json::Value,
    pub gh_created_at: String,
}

#[cfg(feature = "ssr")]
impl From<SqlOpenedPullRequestInfo>
    for ClientOpenedPullRequestInfo
{
    fn from(value: SqlOpenedPullRequestInfo) -> Self {

        ClientOpenedPullRequestInfo {
            github_id: value.github_id,
            url: value.url,
            title: value.title,
            author: value.author,
            author_url: value.author_url,
            labels: value.labels,
            gh_created_at: value
                .gh_created_at
                .expect("a date, because the query was for valid date ranges")
                .format(&crate::issue_date::issue_date_format)
                .expect("a valid format"),
        }
    }
}

// Opened Issues

#[cfg(feature = "ssr")]
#[derive(Debug, sqlx::FromRow)]
struct SqlOpenedIssueInfo {
    github_id: String,
    url: String,
    title: String,
    author: String,
    author_url: String,
    labels: serde_json::Value,
    gh_created_at: Option<time::Date>,
}

#[server]
pub async fn get_opened_issues(
    date: time::Date,
) -> Result<Vec<ClientOpenedIssueInfo>, ServerFnError> {
    let pool = pool()?;

    // one week
    let start_date =
        date - Duration::from_secs(60 * 60 * 24 * 7);

    let results: Vec<SqlOpenedIssueInfo> = sqlx::query_as!(
        SqlOpenedIssueInfo,
        "SELECT
        github_id,
        url,
        title,
        author,
        author_url,
        labels,
        gh_created_at
FROM new_github_issue
WHERE gh_created_at BETWEEN ? AND ?
ORDER BY gh_created_at DESC",
        Some(start_date),
        Some(date),
    )
    .fetch_all(&pool)
    .await?;

    Ok(results
        .into_iter()
        .map(|v| ClientOpenedIssueInfo::from(v))
        .collect())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientOpenedIssueInfo {
    pub github_id: String,
    pub url: String,
    pub title: String,
    pub author: String,
    pub author_url: String,
    pub labels: serde_json::Value,
    pub gh_created_at: String,
}

#[cfg(feature = "ssr")]
impl From<SqlOpenedIssueInfo> for ClientOpenedIssueInfo {
    fn from(value: SqlOpenedIssueInfo) -> Self {

        ClientOpenedIssueInfo {
            github_id: value.github_id,
            url: value.url,
            title: value.title,
            author: value.author,
            author_url: value.author_url,
            labels: value.labels,
            gh_created_at: value
                .gh_created_at
                .expect("a date, because the query was for valid date ranges")
                .format(&crate::issue_date::issue_date_format)
                .expect("a valid format"),
        }
    }
}
