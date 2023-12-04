use clap::Parser;

use crate::util::GitCommand;

#[derive(Parser, Debug)]
pub struct Init {}

impl Init {
    pub fn run(&self) -> anyhow::Result<()> {
        GitCommand::new("init")
            .status()?;
        Ok(())
    }
}