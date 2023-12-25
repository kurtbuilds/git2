use clap::Parser;

use crate::util::GitCommand;

#[derive(Parser, Debug)]
pub struct Status {}

impl Status {
    pub fn run(self) -> anyhow::Result<()> {
        GitCommand::new("status")
            .status()?;
        Ok(())
    }
}