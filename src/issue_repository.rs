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
    pub login: String,
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
    pub title: String,
    pub author: User,
    pub assignee: Option<User>,
    pub issue_type: IssueType,
    pub state: IssueState,
}
