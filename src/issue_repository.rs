use crate::todofinder::{IssueType, ToDo};
use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

#[async_trait]
pub trait IssueBoard {
    async fn get_issues(&self) -> Vec<Issue>;
    async fn get_issue(&self, name: &str) -> Issue;
    async fn add_issue(&self, issue: ToDo) -> Result<(), Box<dyn Error>>;
    async fn update_issue(&self, name: &str) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct User {
    #[serde(rename = "login")]
    pub name: String,
}
#[derive(Debug, Deserialize)]
pub struct Label {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueState {
    Open,
    Closed,
    Reopened,
}
#[derive(Deserialize)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    #[serde(rename = "user")]
    pub author: User,
    pub assignee: Option<User>,
    #[serde(rename = "labels")]
    issue_type: Vec<Label>,
    state: String,
}

impl Issue {
    pub fn state(&self) -> IssueState {
        match self.state.as_str() {
            "open" => IssueState::Open,
            "closed" => IssueState::Closed,
            "reopened" => IssueState::Reopened,
            _ => IssueState::Open,
        }
    }

    pub fn issue_type(&self) -> IssueType {
        let mut issue_type = IssueType::Other;
        for label in &self.issue_type {
            match label.name.as_str() {
                "bug" => issue_type = IssueType::Bug,
                "improvement" => issue_type = IssueType::Improvement,
                "feature" => issue_type = IssueType::Feature,
                _ => {}
            }
        }
        issue_type
    }
}
