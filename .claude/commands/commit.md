---
description: "Commit work in progress to the current worker branch with intelligent commit structuring"
---

# Commit Work in Progress

## Context
- Current git status: !git status
- Current git diff: !git diff
- Recent commit history: !git log --oneline -5

## Your task

1. **Run pre-commit verification steps**:
   - !just check
   - !just test  
   - !just clippy
   
   **STOP AND REPORT IF ANY ISSUES ARE FOUND. DO NOT ATTEMPT TO FIX ISSUES.**

2. **Commit all changes systematically**:
   - Review all staged and unstaged changes
   - Add untracked files that should be committed
   - Create logical, well-described commits using multiple commits if needed
   - Follow the repository's existing commit message style
   - Stage and commit all changes systematically

3. **Run post-commit formatting**:
   - !cargo fmt
   - !git diff --quiet || (git add -A && git commit -m "Apply cargo fmt")

Create meaningful commit messages that describe what was changed and why. If there are many different types of changes, split them into separate logical commits.