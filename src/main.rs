use clap::{Parser, Subcommand};
use crate::util::GitCommand;

mod command;
mod util;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[clap(long, short, global = true)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a migration. Supports auto-generation based on models detected in the codebase.
    #[clap(visible_alias("co"))]
    Checkout(command::Checkout),
    #[clap(visible_alias("c"))]
    Commit(command::Commit),
    #[clap(visible_alias("s"))]
    Status(command::Status),
    #[clap(visible_alias("i"))]
    Ignore(command::Ignore),
    Reset(command::Reset),
    #[clap(external_subcommand)]
    Other(Vec<String>),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    use Command::*;
    match cli.command {
        Checkout(c) => c.run(),
        Commit(c) => c.run(),
        Status(c) => c.run(),
        Ignore(c) => c.run(),
        Reset(c) => c.run(),
        Other(c) => {
            GitCommand::args(c).status()
        },
    }
}