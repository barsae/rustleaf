---
description: "Rebase current work-in-progress over latest main branch changes"
---

# Rebase Work Over Main

Update your work-in-progress with latest main branch changes without completing the merge workflow.
NOTE: There is no remote origin. This entire process will be local-only using worktrees.

## Context
- Current git status: !git status

## Your task

1. **Check if we need to rebase**:
   - !just need-rebase
   - If "Up to date with main" and no uncommitted changes: Report "Already up to date" and stop
   - If "Up to date with main" but uncommitted changes: Report "No rebase needed, only working directory changes" and stop

2. **Handle uncommitted work if rebasing**:
   - If there are uncommitted changes, make a temporary commit: !just make-temp-stash-commit

3. **Rebase all commits over main**:
   - !git rebase main

4. **Restore uncommitted work**:
   - If a STASH commit was made, convert it back to uncommitted changes: !just pop-temp-stash-commit

5. **Handle conflicts if they occur**:
   - If rebase stops due to conflicts, think through a plan to resolve them
      - Don't be afraid to ask for context if it would help your planning
   - After resolving conflicts: !just rebase-continue
   - If conflicts become too complex: stop and report

Use this when another agent has merged changes that your current work depends on.