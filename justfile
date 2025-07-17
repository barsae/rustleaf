# RustLeaf justfile

rust_flags := "RUSTFLAGS=\"-D warnings\""
current_branch := `basename $(pwd)`

# Run cargo check with warnings as errors
check:
    {{rust_flags}} cargo check

# Run tests with warnings as errors
test:
    {{rust_flags}} cargo test

# Run clippy with warnings as errors
clippy:
    cargo clippy -- -D warnings

# Check if we need to rebase (main has commits we don't)
need-rebase:
    #!/bin/bash
    count=$(git rev-list --count HEAD..main)
    if [ $count -gt 0 ]; then
        echo "Need to rebase: $count commits behind main"
    else
        echo "Up to date with main"
    fi

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