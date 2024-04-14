// Create tissuebox.json if doesn't exist
//
// Pull ToDo's from files
//
// Diff with tissuebox.json
//
// Apply difference to github repository. Save ID of tissues to track them on the repository
//
//

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
