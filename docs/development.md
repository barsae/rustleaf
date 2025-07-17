# RustLeaf Development Guide

## Justfile Commands

This project uses `just` for task automation. Run `just --list` to see all available commands.

### Development Commands

#### `just check`
Run cargo check with warnings treated as errors.

#### `just test` 
Run the full test suite with warnings as errors.

#### `just clippy`
Run clippy linter with warnings as errors.

### Worktree Management

#### `just worktree-status`
Display the status of all worktree branches relative to main. Shows:
- ✅ Branches up to date with main
- ⬆️ Branches ahead of main (ready to merge)  
- ⬇️ Branches behind main (need rebase)
- 🔀 Branches both ahead and behind (diverged)

Example output:
```
Worktree branch status relative to main:
========================================
✅ worker-01 (worker-01): up to date
⬆️ worker-02 (worker-02): 3 commits ahead
⬇️ worker-03 (worker-03): 2 commits behind
```

#### `just need-rebase`
Check if the current branch needs to rebase against main.

#### `just merge-ff`
Fast-forward merge the current worker branch into main (when on main).

#### `just checkout-worker-branch`
Return to the worker branch from main.

### Rebase Automation

#### `just rebase`
Automated rebase workflow that:
- Checks if rebase is needed
- Stashes any uncommitted changes
- Performs rebase against main
- Restores stashed changes
- Handles conflicts gracefully

#### `just rebase-continue`
Continue rebase after resolving conflicts.

### Development Workflow

```bash
# Check status of all worktree branches
just worktree-status

# Standard development cycle
just check test clippy

# Merge workflow (typically for agents)
just need-rebase                # Check if rebase needed
git checkout main               # Take main lease  
just merge-ff                   # Merge worker branch
just checkout-worker-branch     # Release lease
```

## Project Structure

- `src/` - Rust source code
- `specs/` - Language specification documents
- `tests/` - Test files and RustLeaf test programs
- `docs/` - Documentation
- `.claude/` - Claude Code slash commands
- `STATUS.md` - Implementation status matrix