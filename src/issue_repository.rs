use async_trait::async_trait;

use crate::todofinder::{IssueType, ToDo};
use std::{error::Error, future::Future};

#[async_trait]
pub trait IssueBoard {
    async fn get_issues(&self) -> Vec<Issue>;
    async fn get_issue(&self, name: &str) -> Issue;
    async fn add_issue(&self, issue: ToDo) -> Result<(), Box<dyn Error>>;
    async fn update_issue(&self, name: &str) -> Result<(), Box<dyn Error>>;
}

pub struct User {
    login: String,
}

pub enum IssueState {
    Open,
    Closed,
    Reopened,
}

pub struct Issue {
    pub title: String,
    #[serde(rename = "user")]
    pub author: User,
    pub assignee: Option<User>,
    pub issue_type: IssueType,
    pub state: IssueState,
}

impl Issue {
    fn set_type(&self, label_string: &str) {
        self.issue_type = match label_string {
            "feature" => IssueType::Feature,
            "bug" => IssueType::Bug,
            "improvement" => IssueType::Improvement,
            _ => IssueType::Other,
        };
    }
}
