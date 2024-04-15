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
struct Tissue {
    issue: Issue,
    todo: ToDo,
}

#[async_trait]
trait TissueBoxRepository {
    async fn compare_and_apply() -> Result<()>;
    fn get_tissues() -> Result<Vec<Tissue>>;
    fn add_board(issue_board: &dyn IssueBoard) -> Result<()>;
    fn remove_board(board_name: &str) -> Result<&dyn IssueBoard>;
}
