use std::env;
use std::error::Error;

use async_trait::async_trait;
use reqwest::header::{self, HeaderMap, AUTHORIZATION};
use reqwest::Client;

use crate::issue_repository::{Issue, IssueBoard};
use crate::todofinder::ToDo;

#[derive(Debug)]
struct GitHubIntegration {
    project: String,
    org: String,
}

impl GitHubIntegration {
    async fn new(org: &str, project: &str) -> GitHubIntegration {
        GitHubIntegration {
            project: String::from(project),
            org: String::from(org),
        }
    }
}
#[async_trait]
impl IssueBoard for GitHubIntegration {
    async fn get_issues(&self) -> Vec<Issue> {
        let client = get_http_client();
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues",
            self.org, self.project
        );

        let response = client
            .get(&url)
            .bearer_auth(env::var("TISSUE_GITHUB_TOKEN").expect("Token not found"))
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header(header::USER_AGENT, "tissue")
            .send()
            .await
            .expect("Failed to send request");

        if response.status().is_success() {
            let issues = response
                .json::<Vec<Issue>>()
                .await
                .expect("Failed to parse JSON");
            issues
        } else {
            eprintln!(
                "Failed to fetch issues: {:?}",
                response.text().await.unwrap()
            );
            panic!("Failed to fetch issues");
        }
    }
    async fn get_issue(&self, number: u32) -> Issue {}
    async fn add_issue(&self, issue: ToDo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    async fn update_issue(&self, name: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

fn get_http_client() -> Client {
    let access_token = match env::var("TISSUE_GITHUB_TOKEN") {
        Ok(token) => token,
        Err(err) => panic!("No GitHub token found: {}", err),
    };
    let mut headers = HeaderMap::new();
    let mut token = "Bearer ".to_string();
    token.push_str(&access_token);
    headers.insert(
        AUTHORIZATION,
        header::HeaderValue::from_str(&token).unwrap(),
    );
    return reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::issue_repository::IssueBoard;

    use super::GitHubIntegration;
    #[tokio::test]
    async fn can_connect_with_github_client() {
        let client = GitHubIntegration::new("OthelloEngineer", "tissue").await;
    }

    #[tokio::test]
    async fn can_retrieve_test_issue() {
        let client = GitHubIntegration::new("OthelloEngineer", "tissue").await;
        let issues = client.get_issues().await;
        let test_issue = issues
            .iter()
            .find(|issue| issue.title == "test issue".to_string())
            .unwrap();
        assert_eq!(test_issue.author.name, "OthelloEngineer".to_string())
    }
}
