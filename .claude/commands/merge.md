---
description: "Complete the full Worktree Agent Flow with intelligent commit structuring"
---

# Complete Merge Workflow

Execute the complete Worktree Agent Flow to integrate finished work.

## Context
- Current git status: !git status

## Your task

1. **Handle uncommitted work first**:
   - If there are uncommitted changes, run the `/commit` command first
   - Return here after committing

2. **Rebase if necessary**:
   - Check if main has commits we don't: !just need-rebase
   - If behind main, run `/rebase` command to update our branch
   - Return here after rebasing

3. **Take lease on main branch**:
   - !git checkout main
   - If this fails, another agent has the lease - stop and report

4. **Fast-forward merge from worker branch**:
   - !just merge-ff
   - If this fails, main - stop and report

5. **Release lease - return to worker branch**:
   - !just checkout-worker-branch

The worker branch should already be up-to-date with main after the merge.