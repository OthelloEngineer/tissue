use std::env;
use std::error::Error;

use async_trait::async_trait;
use reqwest::header::{self, HeaderMap};
use reqwest::Client;

use crate::issue_repository::{Issue, IssueBoard};
use crate::todofinder::ToDo;

#[derive(Debug)]
struct GitHubIntegration {
    project: String,
    org: String,
    client: Client,
}

impl GitHubIntegration {
    async fn new(org: &str, project: &str) -> GitHubIntegration {
        let access_token = match env::var("TISSUE_GITHUB_TOKEN") {
            Ok(token) => token,
            Err(err) => panic!("No GitHub token found: {}", err),
        };
        let mut headers = HeaderMap::new();
        let mut token = "Bearer ".to_string();
        token.push_str(&access_token);
        headers.insert(
            header::HeaderName::from_static("Authorization"),
            header::HeaderValue::from_str(&token).unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        GitHubIntegration {
            project: String::from(project),
            org: String::from(org),
            client,
        }
    }
}
#[async_trait]
impl IssueBoard for GitHubIntegration {
    async fn get_issues(&self) -> Vec<Issue> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues",
            self.org, self.project
        );
        let reponse_req = self.client.get(url);
        println!("{:?}", self.client);
        println!("{:?}", reponse_req);
        let response = reponse_req.send().await.unwrap();
        println!("{:?}", response);
        let issues: Vec<Issue> = response.json::<Vec<Issue>>().await.unwrap();
        return issues;
    }
    async fn get_issue(&self, name: &str) -> Issue {
        todo!()
    }
    async fn add_issue(&self, issue: ToDo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    async fn update_issue(&self, name: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
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
        assert_eq!(test_issue.author.login, "OthelloEngineer".to_string())
    }
}
