// Create tissuebox.json if doesn't exist
//
// Pull ToDo's from files
//
// Diff with tissuebox.json
//
// Apply difference to github repository. Save ID of tissues to track them on the repository
//
//
//diff --git a/src/integration-test.rs b/src/integration-test.rs
//index 31d7315..1aee8f7 100644
//--- a/src/integration-test.rs
//+++ b/src/integration-test.rs
//@@ -1,6 +1,9 @@
// #[cfg(test)]
// mod tests {
//-    use crate::{filereader, githandler, todofinder::{self, Submission}};
//+    use crate::{
//+        filereader, githandler,
//+        todofinder::{self, Submission},
//+    };
//
//     use super::*;
//     use std::fs;
//@@ -10,7 +13,7 @@ mod tests {
//             Ok(files) => files,
//             Err(e) => panic!("Error: {}", e),
//         };
//-        let mut submissions : Vec<Submission> = Vec::new();
//+        let mut submissions: Vec<Submission> = Vec::new();
//         for file in files {
//             for line in file.lines {
//                 let issuer = githandler::blame_user_from_line(&file.file_path, line.0).unwrap();

use async_trait::async_trait;
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{
    issue_repository::{Issue, IssueBoard},
    todofinder::ToDo,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tissue {
    pub issue: Issue,
    pub todo: ToDo,
}

#[async_trait]
pub trait TissueBoxRepository {
    fn add_tissues(self, tissues: Vec<Tissue>) -> Result<()>;
    fn remove_tissues(self, tissues: Vec<&Tissue>) -> Result<Vec<Tissue>>;
    fn update_tissues(self, tissues: Vec<Tissue>) -> Result<()>;
    fn get_tissues(self) -> Result<Vec<Tissue>>;
    fn add_board(self, issue_board: &dyn IssueBoard) -> Result<()>;
    fn remove_board(self, board_name: &str) -> Result<&dyn IssueBoard>;
}
