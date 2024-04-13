use std::env;
use std::error::Error;

use async_trait::async_trait;
use octocrab::Octocrab;

use crate::issue_repository::{Issue, IssueBoard};
use crate::todofinder::{parse_issue_type, ToDo};

#[derive(Debug)]
struct GitHubIntegration {
    project: String,
    org: String,
    instance: Octocrab,
}

impl GitHubIntegration {
    async fn new(org: &str, project: &str) -> GitHubIntegration {
        let access_token = match env::var("TISSUE_GITHUB_TOKEN") {
            Ok(token) => token,
            Err(err) => panic!("No GitHub token found: {}", err),
        };
        let instance = Octocrab::builder()
            .personal_token(access_token)
            .build()
            .expect("Could not find access token");

        return GitHubIntegration {
            project: String::from(org),
            org: String::from(org),
            instance,
        };
    }
}
#[async_trait]
impl IssueBoard for GitHubIntegration {
    async fn get_issues(&self) -> Vec<Issue> {
        let crab_issues = self
            .instance
            .issues(self.org.clone(), self.project.clone())
            .list()
            .send()
            .await
            .unwrap();

        let issues = crab_issues
            .into_iter()
            .map(|issue| {
                let issue_type = parse_issue_type(&issue.title);
                Issue {
                    description: issue.title,
                    issue_type,
                    author: issue.user.login,
                    assigned: Some(issue.assignee.unwrap().login),
                }
            })
            .collect();
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
    use super::GitHubIntegration;
    #[tokio::test]
    async fn can_connect_with_github_client() {
        let client = GitHubIntegration::new("OthelloEngineer", "tissue").await;
    }
}
