
# RustLeaf

## Specs

The specifications live at `./specs/*.md`.

## Pre-Commit Verification

Before committing changes, always run these verification steps:

1. **Compile Check**: `RUSTFLAGS="-D warnings" cargo check`
   - Ensures the code compiles without errors or warnings

2. **Run Tests**: `RUSTFLAGS="-D warnings" cargo test`
   - Verifies all tests pass without warnings

3. **Linting**: `cargo clippy -- -D warnings`
   - Checks for common mistakes and style issues
   - **Policy**: All warnings must be fixed before committing

## Post-Commit Formatting

After committing, run formatting as a separate commit:

1. **Format Code**: `cargo fmt`
2. **Check if formatting changed anything**: `git diff`
3. **If there are changes**: `git add -A && git commit -m "Apply cargo fmt"`

## Git Workflow for Agents - Worktree Agent Flow

### Overview
Each agent works in their own persistent worktree with a dedicated branch. The **Worktree Agent Flow** follows a rebase-squash pattern with Git's built-in checkout exclusivity providing the lease mechanism for main branch integration.

### Setup
- Each agent has their own worktree: `worker-01`, `worker-02`, etc.
- Worktrees are created from the main branch
- Each agent maintains their own long-standing branch

### Workflow Steps

#### 1. Work in Agent Branch
```bash
# Agent works in their dedicated worktree/branch
cd /path/to/worker-XX
# Make changes, commits, etc.
```

#### 2. Rebase-Squash Against Main
```bash
# Update main reference
git fetch origin main:main

# Interactive rebase to squash commits
git rebase -i main

# Alternative: squash all commits into one
git reset --soft main
git commit -m "Descriptive commit message"
```

#### 3. Take Lease on Main Branch
```bash
# Switch to main branch
git checkout main

# Fast-forward merge from agent branch
git merge --ff-only worker-XX
```

#### 4. Release Lease - Return to Agent Branch
```bash
# Return to agent's persistent branch
git checkout worker-XX

# Update agent branch to match main
git reset --hard main
```
