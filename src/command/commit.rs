use clap::Parser;

use crate::util::{git_root, GitCommand};

#[derive(Parser, Debug)]
pub struct Commit {
    paths: Vec<String>,
}

impl Commit {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut c = GitCommand::new("add");
        for path in &self.paths {
            c = c.arg(path);
        }
        if self.paths.is_empty() {
            let path = git_root().to_str().unwrap();
            c = c.arg(path);
        }
        c.status()?;
        GitCommand::new("commit")
            .status()?;
        Ok(())
    }
}