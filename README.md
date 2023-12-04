# `git2`

This is an experiment in making a git that's much easier to use.

What it does:
- *No staging*: With no staging, there's also no `add` command. Just commit things. Commands that modify the working directory or staging will first trigger saving
  all changes to the stash, meaning you can entirely forget about the concept of staging.
- *Support stacked diffs (PRs)*: The tool aims to make submitted stacked PRs to Github a simple workflow.

What stays the same:
- *No name changes*. `git2` removes or simplifies concepts from git, but the basic concepts stay the same (branch, pr, checkout, etc.)
  That means there's minimal learning curve if you already know git.
- *100% git compatibility*. `git2` is a command line wrapper around `git` itself, so you can always fall back to `git` if you need to.

# Workflow

Here's an example workflow.

```bash
git2 init
touch README.md
# Commit the readme. You'll be prompted to enter a message.
# Unlike git, you don't need to add the file first.
git2 commit
# Create a new branch
git2 checkout -b my-feature 
echo "print('hello world')" > hello.py
# Commit the python file
git2 commit
```

# Roadmap

- [ ] git2 pull needs to stash before running.
- [ ] git2 pr to create a pull request.
- [ ] git2 pr is smart enough to stack diffs.
- [ ] Find a way to make resolving merge conflicts in stacked diffs easier.
- [ ] git2 merge does a rebase merge.
- [ ] git2 squash does a rebase --squash merge.

I think if we do rebase & --squash, we'll also want a way to update stacked PRs and whatnot.
