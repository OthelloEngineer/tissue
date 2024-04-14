use std::env;

use async_trait::async_trait;
use color_eyre::eyre::Result;
use reqwest::header::{self, HeaderMap, AUTHORIZATION};
use reqwest::Client;

use crate::issue_repository::{Issue, IssueBoard, IssueState, IssueUpdateRequest};
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
    async fn get_issues(&self) -> Result<Vec<Issue>> {
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
            Ok(issues)
        } else {
            println!(
                "Failed to fetch issues: {:?}",
                response.text().await.unwrap()
            );
            Err(color_eyre::Report::msg(
                "F in the chat, didn't find any issues :(",
            ))
        }
    }
    async fn get_issue(&self, number: u32) -> Result<Issue> {
        let client = get_http_client();
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}",
            self.org, self.project, number
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
            let issue = response
                .json::<Issue>()
                .await
                .expect("Failed to parse JSON");
            Ok(issue)
        } else {
            println!(
                "Failed to fetch issue: {:?}",
                response.text().await.unwrap()
            );
            Err(color_eyre::Report::msg(
                "F in the chat, didn't find any issues :(",
            ))
        }
    }
    async fn add_issue(&self, issue: Issue) -> Result<u32> {
        let client = get_http_client();
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues",
            self.org, self.project
        );

        let data = serde_json::to_string(&issue).expect("Failed to serialize issue");
        let response = client
            .post(&url)
            .bearer_auth(env::var("TISSUE_GITHUB_TOKEN").expect("Token not found"))
            .body(data)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header(header::USER_AGENT, "tissue")
            .send()
            .await
            .expect("Failed to send request");

        if response.status().is_success() {
            let issue = response
                .json::<Issue>()
                .await
                .expect("Failed to parse JSON");
            Ok(issue.number.expect("Issue number not found"))
        } else {
            println!(
                "Failed to create issue: {:?}",
                response.text().await.unwrap()
            );
            Err(color_eyre::Report::msg("Couldn't create issue :("))
        }
    }
    async fn update_issue(&self, number: u32, update: &IssueUpdateRequest) -> Result<()> {
        let client = get_http_client();
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}",
            self.org, self.project, number
        );

        let data = match update {
            IssueUpdateRequest::State(state) => {
                format!("{{\"state\":\"{}\"}}", state.as_str())
            }
            IssueUpdateRequest::Assignee(user) => {
                format!("{{\"assignees\":[\"{}\"]}}", user.name)
            }
            IssueUpdateRequest::Title(title) => {
                format!("{{\"title\":\"{}\"}}", title)
            }
            IssueUpdateRequest::IssueType(issue_type) => {
                format!("{{\"labels\":[\"{}\"]}}", issue_type.as_str())
            }
            IssueUpdateRequest::Delete() => {
                format!("{{\"state\":\"closed\"}}")
            }
        };

        let response = client
            .patch(&url)
            .bearer_auth(env::var("TISSUE_GITHUB_TOKEN").expect("Token not found"))
            .body(data)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header(header::USER_AGENT, "tissue")
            .send()
            .await
            .expect("Failed to send request");
        if response.status().is_success() {
            println!("{:?}", response.text().await.unwrap());
            Ok(())
        } else {
            println!(
                "Failed to update issue: {:?}",
                response.text().await.unwrap()
            );
            Err(color_eyre::Report::msg("Couldn't update issue :("))
        }
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
    use crate::issue_repository::{Issue, IssueBoard, Label, User};

    use super::GitHubIntegration;
    #[tokio::test]
    async fn can_connect_with_github_client() {
        let client = GitHubIntegration::new("OthelloEngineer", "tissue").await;
    }

    #[tokio::test]
    async fn can_retrieve_test_issue() {
        let client = GitHubIntegration::new("OthelloEngineer", "tissue").await;
        let issues: Vec<Issue> = client.get_issues().await.unwrap();
        let test_issue = issues
            .iter()
            .find(|issue| issue.title == "test issue")
            .unwrap();
        assert_eq!(test_issue.author.name, "OthelloEngineer".to_string())
    }

    #[tokio::test]
    async fn can_retrieve_test_issue_by_number() {
        let client = GitHubIntegration::new("OthelloEngineer", "tissue").await;
        let test_issue = client.get_issue(1).await.unwrap();
        assert_eq!(test_issue.title, "test issue".to_string())
    }

    #[tokio::test]
    async fn can_update_test_issue() {
        let client = GitHubIntegration::new("OthelloEngineer", "tissue").await;
        let test_number = 3;
        let test_issue = client.get_issue(test_number).await.unwrap();

        let issue_update_request = match test_issue.title.as_str() {
            "test issue2 blue" => super::IssueUpdateRequest::Title("test issue2 green".to_string()),
            "test issue2 green" => super::IssueUpdateRequest::Title("test issue2 blue".to_string()),
            _ => panic!("test name not recognized"),
        };

        client.update_issue(3, &issue_update_request).await.unwrap();
        let updated_issue = client.get_issue(test_number).await.unwrap();
        assert_eq!(updated_issue.title, issue_update_request.as_str())
    }

    #[cfg(feature = "manual_reversible")]
    #[tokio::test]
    async fn can_create_issue() {
        let client = GitHubIntegration::new("OthelloEngineer", "tissue").await;
        let new_issue = Issue {
            title: "automatic test issue".to_string(),
            assignee: None,
            author: User {
                name: "OthelloEngineer".to_string(),
            },
            issue_type: vec![Label {
                name: "bug".to_string(),
            }],
            state: "open".to_string(),
            number: None,
        };
        let issue_number = client.add_issue(new_issue).await.unwrap();
        let issue = client.get_issue(issue_number).await.unwrap();
        assert_eq!(issue.title, "automatic test issue".to_string())
    }
}
