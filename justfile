# RustLeaf justfile

rust_flags := "RUSTFLAGS=\"-D warnings\""
current_branch := `basename $(pwd)`

# Check if test directories changed and trigger rebuild if needed
check-test-dirs:
    #!/bin/bash
    hash_file="target/.test-dirs-hash"
    current_hash=$(tree tests/integration tests/unit 2>/dev/null | shasum -a 256 | cut -d' ' -f1)

    # Ensure target directory exists
    mkdir -p target

    stored_hash=""
    if [ -f "$hash_file" ]; then
        stored_hash=$(cat "$hash_file")
    fi

    if [ "$current_hash" != "$stored_hash" ]; then
        echo "Test directory structure changed, triggering rebuild..."
        touch tests/integration/mod.rs
    fi

    echo "$current_hash" > "$hash_file"

# Run check, test, and clippy with warnings as errors
test: check-test-dirs
    {{rust_flags}} cargo check
    {{rust_flags}} cargo test
    cargo clippy -- -D warnings

# Generate test summary from integration tests
test-summary:
    python3 scripts/generate-test-summary.py

# Check if we need to rebase (main has commits we don't)
need-rebase:
    #!/bin/bash
    count=$(git rev-list --count HEAD..main)
    if [ $count -gt 0 ]; then
        echo "Need to rebase: $count commits behind main"
    else
        echo "Up to date with main"
    fi

# Show status of all worktree branches relative to main
worktree-status:
    #!/bin/bash
    echo "Worktree branch status relative to main:"
    echo "========================================"

    # Get all worktree paths and branches
    git worktree list --porcelain | grep -E '^worktree|^branch' | while read -r line; do
        if [[ $line == worktree* ]]; then
            worktree_path=${line#worktree }
            read -r branch_line
            branch_name=${branch_line#branch refs/heads/}

            # Skip main branch
            if [ "$branch_name" = "main" ]; then
                continue
            fi

            # Get ahead/behind counts
            ahead=$(git rev-list --count main..$branch_name 2>/dev/null || echo "0")
            behind=$(git rev-list --count $branch_name..main 2>/dev/null || echo "0")

            # Format branch name with path
            branch_display="$branch_name ($(basename "$worktree_path"))"

            # Show status
            if [ "$ahead" -eq 0 ] && [ "$behind" -eq 0 ]; then
                echo "✅ $branch_display: up to date"
            elif [ "$behind" -eq 0 ]; then
                echo "⬆️  $branch_display: $ahead commits ahead"
            elif [ "$ahead" -eq 0 ]; then
                echo "⬇️  $branch_display: $behind commits behind"
            else
                echo "🔀 $branch_display: $ahead ahead, $behind behind"
            fi
        fi
    done

# Fast-forward merge from worker branch
merge-ff:
    git merge --ff-only {{current_branch}}

# Checkout worker branch
checkout-worker-branch:
    git checkout {{current_branch}}

# Make temporary stash commit for rebase
make-temp-stash-commit:
    #!/bin/bash
    if ! git diff --quiet || ! git diff --cached --quiet; then
        git add -A && git commit -m "STASH: WIP before rebase"
    fi

# Pop temporary stash commit back to working directory
pop-temp-stash-commit:
    #!/bin/bash
    if [ "$(git log -1 --pretty=format:%s)" = "STASH: WIP before rebase" ]; then
        git reset HEAD~1
    else
        echo "No temp stash commit found, ignoring"
    fi

# Automated rebase workflow
rebase:
    #!/bin/bash
    # Check if rebase needed
    count=$(git rev-list --count HEAD..main)
    if [ $count -eq 0 ]; then
        echo "Already up to date with main"
        exit 0
    fi

    # Stash any changes
    just make-temp-stash-commit

    # Attempt rebase
    if git rebase main; then
        echo "Rebase successful"
        just pop-temp-stash-commit
    else
        echo "Conflicts detected. Run 'just rebase-continue' after resolving."
        exit 1
    fi

# Continue rebase after resolving conflicts
rebase-continue:
    git add -A
    git rebase --continue
    just pop-temp-stash-commit

# Commit work in progress to current worker branch
commit:
    #!/bin/bash
    # Check if there are changes to commit
    if git diff --quiet && git diff --cached --quiet; then
        echo "No changes to commit."
        exit 0
    fi

    #!/bin/bash
    if ! just test; then
        echo "❌ Tests failed. Please fix issues before committing."
        exit 1
    fi

    cargo fmt
    echo "Claude is committing..."
    claude -p "/commit"

# Merge current branch into main
merge:
    #!/bin/bash
    # Verify no changes in progress
    if ! git diff --quiet || ! git diff --cached --quiet; then
        echo "Cannot merge: uncommitted changes detected"
        echo "Please commit or stash your changes first"
        exit 1
    fi
    git checkout main
    just merge-ff
    just checkout-worker-branch
