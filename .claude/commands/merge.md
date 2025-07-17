# Complete Merge Workflow

Execute the complete Worktree Agent Flow to integrate finished work.

## Your task

1. **Handle uncommitted work first**:
   - Check git status: !git status
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
   - If this fails, stop and report

5. **Release lease - return to worker branch**:
   - !just checkout-worker-branch
   - The worker branch will now be up-to-date with main after the merge - no further action needed 

6. **Provide summary**:
   - Write a *very* brief summary
   - !just worktree-status
