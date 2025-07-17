---
description: "Complete the full Worktree Agent Flow with intelligent commit structuring"
---

# Complete Merge Workflow

Execute the complete Worktree Agent Flow to integrate finished work.

## Context
- Worker branch: !basename $(pwd)
- Current git status: !git status
- Commits since main: !git log --oneline main..HEAD
- Recent main history: !git log --oneline main -5

## Your task

1. **Handle uncommitted work first**:
   - If there are uncommitted changes, run the `/commit` command first
   - Return here after committing

2. **Review commits since main**:
   - Review all commits since main branch
   - Ensure commits have good messages and represent logical changes
   - Proceed with merge as-is (no restructuring needed)
   - If you feel there is a concern - stop and report

3. **Take lease on main branch**:
   - !git checkout main
   - If this fails, another agent has the lease - stop and report

4. **Fast-forward merge from worker branch**:
   - !just merge-ff
   - If this fails, main has diverged - stop and report

5. **Release lease - return to worker branch**:
   - !just checkout-worker-branch

The worker branch should already be up-to-date with main after the merge.