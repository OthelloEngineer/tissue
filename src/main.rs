use color_eyre::eyre::{Ok, Result};

pub mod filereader;
pub mod githandler;
pub mod github_integration;
pub mod issue_repository;
#[path = "integration-test.rs"]
pub mod tests;
pub mod tissuebox_file;
pub mod tissuebox_repository;
pub mod todofinder;

fn main() -> Result<()> {
    color_eyre::install()?;
    Ok(())
}
