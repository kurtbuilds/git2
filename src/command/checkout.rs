use std::sync::OnceLock;

use anyhow::Result;
use clap::Parser;

use crate::util::{git_current_branch, git_current_hash, git_root, GitCommand};

#[derive(Parser, Debug)]
pub struct Checkout {
    ref_name: String,
    #[clap(long, short)]
    branch: bool,
}

fn make_stash_name(ref_name: &str) -> String {
    format!("working:{}", ref_name)
}

static STASH_OUTPUT_REGEX: OnceLock<regex::Regex> = OnceLock::new();

fn stash_output_regex() -> &'static regex::Regex {
    STASH_OUTPUT_REGEX.get_or_init(|| {
        regex::Regex::new(r"stash@\{(\d+)\}: .+: (.+?)$").unwrap()
    })
}

fn find_number_in_stash_output(output: &str, filter: &str) -> Option<String> {
    for line in output.lines() {
        let caps = stash_output_regex().captures(line)?;
        let name = caps.get(2)?.as_str();
        if name == filter {
            let number = caps.get(1)?;
            return Some(number.as_str().to_string());
        }
    }
    None
}

impl Checkout {
    pub fn run(self) -> Result<()> {
        // git command to unstage everything
        // $ git reset HEAD -- .
        let stash_name = {
            let branch_name = git_current_branch();
            let on_branch = branch_name != "HEAD";
            if on_branch {
                make_stash_name(&branch_name)
            } else {
                let hash_name = git_current_hash();
                make_stash_name(&hash_name)
            }
        };
        let root = git_root().to_str().unwrap();

        // stage everything. this includes files that haven't been added yet.
        GitCommand::new("add")
            .arg(root)
            .status()?;
        // stash those changes in the stash
        GitCommand::new("stash")
            .arg("push")
            .arg("-m")
            .arg(&stash_name)
            .status()?;
        if self.branch {
            GitCommand::new("checkout")
                .arg("-b")
                .arg(&self.ref_name)
                .status()?;
            return Ok(());
        } else {
            GitCommand::new("checkout")
                .arg(&self.ref_name)
                .status()?;
            let stash_name = make_stash_name(&self.ref_name);
            let output = GitCommand::new("stash")
                .arg("list")
                .output()?;
            let number = find_number_in_stash_output(
                std::str::from_utf8(&output.stdout).unwrap(),
                &stash_name,
            );
            let Some(number) = number else {
                println!("No stashed changes found.");
                return Ok(());
            };
            GitCommand::new("stash")
                .arg("pop")
                .arg(&number)
                .status()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_output() {
        let output = r#"stash@{0}: On master: stash:master
stash@{1}: On master: master
stash@{2}: WIP on foo: 256c220 remove .vscode
stash@{3}: On master: stash:master"#;
        let number = find_number_in_stash_output(output, "stash:master");
        assert_eq!(number, Some("0".to_string()));
    }
}