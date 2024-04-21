use core::fmt;
use std::{fmt::Formatter, fs};

use crate::{
    githandler,
    issue_repository::IssueBoard,
    tissuebox_repository::{Tissue, TissueBoxRepository},
};
use async_trait::async_trait;
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TissueBoxFile {
    pub path: String,
    pub last_commit: String,
    pub boards: Vec<String>,
    pub tissues: Vec<Tissue>,
}

impl TissueBoxFile {
    pub fn new(path: String, boards: Vec<Box<dyn IssueBoard>>) -> TissueBoxFile {
        let last_commit = String::from("None");
        let boards = boards
            .iter()
            .map(|board| board.get_repository_name())
            .collect();
        let tissue_box = TissueBoxFile {
            path: path.clone(),
            last_commit,
            boards,
            tissues: vec![],
        };

        if fs::metadata(&path).is_err() {
            //TODO improvement: This dialoguer should be moved to UI-layer
            let user_input = dialoguer::Confirm::new()
                .with_prompt("No Tissue Box was found. \n Do you want to create a new Tissue Box?")
                .interact()
                .unwrap();
            if !user_input {
                println!("Exiting due to user input");
                std::process::exit(0);
            }
            fs::write(
                format!("{}/tissue_box.json", &path),
                serde_json::to_string(&tissue_box)
                    .expect("couldn't convert TissueBox Struct to json :/"),
            )
            .expect("Failed to create file");
        }
        tissue_box
    }
}

#[async_trait]
impl TissueBoxRepository for TissueBoxFile {
    fn add_tissues(self, tissues: Vec<&Tissue>) -> Result<()> {
        let file_path = format!("{}/tissue_box.json", &self.path);
        let file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(file_path.clone())
            .expect("Failed to open file");

        let mut tissue_box: TissueBoxFile = serde_json::from_reader(&file).expect("Failed to read");
        tissue_box.tissues.extend(tissues);
        tissue_box.last_commit =
            githandler::get_last_commit_hash().expect("Failed to get commit hash");

        fs::write(
            file_path,
            serde_json::to_string(&tissue_box).expect("Failed to convert to json"),
        )
        .expect("Failed to update Tissue Box file");
        Ok(())
    }
    fn remove_tissues(self, tissues: Vec<Tissue>) -> Result<Vec<Tissue>> {
        let file_path = format!("{}/tissue_box.json", &self.path);
        let file = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(file_path.clone())
            .expect("Failed to open file");

        let mut tissue_box: TissueBoxFile = serde_json::from_reader(&file).expect("Failed to read");
        let mut removed_tissues = Vec::new();
        for tissue in tissues {
            let idx = tissue_box
                .tissues
                .iter()
                .position(|t| t.issue.number == tissue.issue.number)
                .expect("Tissue not found");
            removed_tissues.push(tissue_box.tissues.remove(idx));
        }

        fs::write(
            file_path,
            serde_json::to_string(&tissue_box).expect("Failed to convert to json"),
        )
        .expect("Failed to update Tissue Box file");
        Ok(removed_tissues)
    }
    fn update_tissues(self, tissues: Vec<Tissue>) -> Result<()> {
        self.removed_tissues(tissues.clone())?;
        self.add_tissues(tissues)?;
    }
    fn get_tissues() -> Result<Vec<Tissue>> {
        Ok(vec![])
    }
    fn add_board(issue_board: &dyn IssueBoard) -> Result<()> {
        Ok(())
    }
    fn remove_board(board_name: &str) -> Result<&dyn IssueBoard> {
        todo!()
    }
}
