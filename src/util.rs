use std::path::PathBuf;
use std::sync::OnceLock;

static GIT_ROOT: OnceLock<PathBuf> = OnceLock::new();
static GIT_CURRENT_BRANCH: OnceLock<String> = OnceLock::new();
static GIT_CURRENT_HASH: OnceLock<String> = OnceLock::new();

pub fn git_root() -> &'static PathBuf {
    GIT_ROOT.get_or_init(|| {
        let output = std::process::Command::new("git")
            .args(&["rev-parse", "--show-toplevel"])
            .output()
            .expect("Failed to run git");
        let path = std::str::from_utf8(&output.stdout)
            .expect("Failed to parse git output")
            .trim();
        std::path::PathBuf::from(path)
    })
}

pub fn git_current_branch() -> &'static String {
    GIT_CURRENT_BRANCH.get_or_init(|| {
        let output = std::process::Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .expect("Failed to run git");
        let branch = std::str::from_utf8(&output.stdout)
            .expect("Failed to parse git output")
            .trim();
        branch.to_string()
    })
}

pub fn git_current_hash() -> &'static String {
    GIT_CURRENT_HASH.get_or_init(|| {
        let output = std::process::Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .expect("Failed to run git");
        let hash = std::str::from_utf8(&output.stdout)
            .expect("Failed to parse git output")
            .trim();
        hash.to_string()
    })
}

pub struct GitCommand {
    args: Vec<String>,
}

impl GitCommand {
    pub fn args(args: Vec<String>) -> Self {
        Self { args }
    }

    pub fn new(command: &str) -> Self {
        Self {
            args: vec![command.to_string()],
        }
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn exec(self) -> std::process::Command {
        let mut c =
            std::process::Command::new("git");
        c.args(&self.args);
        // .current_dir(git_root());
        c
    }

    pub fn output(self) -> anyhow::Result<std::process::Output> {
        let output = self.exec()
            .output()?;
        if !output.status.success() {
            anyhow::bail!("Failed to run git: {:?}", output);
        }
        Ok(output)
    }

    pub fn status(self) -> anyhow::Result<()> {
        let status = self.exec()
            .status()?;
        if !status.success() {
            anyhow::bail!("git2 failed to run git: {:?}", status);
        }
        Ok(())
    }
}