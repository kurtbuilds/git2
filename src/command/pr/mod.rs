mod checkout;

pub use checkout::*;

use clap::{Parser, Subcommand};
use crate::prelude::*;

#[derive(Parser, Debug)]
pub struct PullRequest {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(visible_alias("co"))]
    Checkout(Checkout),
}

impl PullRequest {
    pub fn run(self) -> Result {
        use Command::*;
        match self.command {
            Checkout(c) => c.run(),
        }
    }
}