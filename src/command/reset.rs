use clap::Parser;

use crate::util::GitCommand;

#[derive(Parser, Debug)]
pub struct Reset {
    ref_name: Option<String>,
}

impl Reset {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut c = GitCommand::new("reset")
            .arg("--hard");
        if let Some(ref_name) = &self.ref_name {
            c = c.arg(ref_name);
        }
        c.status()?;
        GitCommand::new("clean")
            .arg("-fd")
            .status()?;
        Ok(())
    }
}