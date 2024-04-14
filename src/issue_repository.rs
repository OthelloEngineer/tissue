use crate::todofinder::{IssueType, ToDo};
use async_trait::async_trait;
use color_eyre::eyre::Result;
use serde::Deserialize;
use serde::Serialize;

#[async_trait]
pub trait IssueBoard {
    async fn get_issues(&self) -> Result<Vec<Issue>>;
    async fn get_issue(&self, number: u32) -> Result<Issue>;
    async fn add_issue(&self, issue: Issue) -> Result<u32>;
    async fn update_issue(&self, number: u32, update: &IssueUpdateRequest) -> Result<()>;
}

pub enum IssueUpdateRequest {
    State(IssueState),
    Assignee(User),
    Title(String),
    IssueType(IssueType),
    Delete(),
}
impl IssueUpdateRequest {
    pub fn as_str(&self) -> &str {
        match self {
            IssueUpdateRequest::State(state) => state.as_str(),
            IssueUpdateRequest::Assignee(assignee) => assignee.name.as_str(),
            IssueUpdateRequest::Title(title) => title.as_str(),
            IssueUpdateRequest::IssueType(issue_type) => issue_type.as_str(),
            IssueUpdateRequest::Delete() => "delete",
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct User {
    #[serde(rename = "login")]
    pub name: String,
}
#[derive(Debug, Deserialize, Serialize)]
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
impl IssueState {
    pub fn as_str(&self) -> &str {
        match self {
            IssueState::Open => "open",
            IssueState::Closed => "closed",
            IssueState::Reopened => "reopened",
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    pub number: Option<u32>,
    pub title: String,
    #[serde(rename = "user")]
    pub author: User,
    pub assignee: Option<User>,
    #[serde(rename = "labels")]
    pub issue_type: Vec<Label>,
    pub state: String,
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
