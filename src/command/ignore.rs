use std::fs;
use std::io::{Read, Write};

use clap::Parser;

use crate::util::git_root;

#[derive(Parser, Debug)]
pub struct Ignore {
    paths: Vec<String>,
}

impl Ignore {
    pub fn run(&self) -> anyhow::Result<()> {
        let path = git_root();
        let mut file = fs::File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(path.join(".gitignore"))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let has_newline = buf.ends_with('\n');
        let mut lines = self.paths.join("\n");
        if !has_newline {
            lines.insert(0, '\n');
        }
        lines.push('\n');
        file.write(lines.as_bytes())?;
        Ok(())
    }
}