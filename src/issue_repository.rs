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

pub struct Issue {
    pub author: String,
    pub description: String,
    pub assigned: Option<String>,
    pub issue_type: IssueType,
}
