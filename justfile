# RustLeaf justfile

# Run cargo check with warnings as errors
check:
    RUSTFLAGS="-D warnings" cargo check

# Run tests with warnings as errors
test:
    RUSTFLAGS="-D warnings" cargo test

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
    git merge --ff-only $(basename $(pwd))

# Checkout worker branch
checkout-worker-branch:
    git checkout $(basename $(pwd))

# Make temporary stash commit for rebase
make-temp-stash-commit:
    git add -A && git commit -m "STASH: WIP before rebase"

# Pop temporary stash commit back to working directory
pop-temp-stash-commit:
    git reset HEAD~1

# Continue rebase after resolving conflicts
rebase-continue:
    git add -A && git rebase --continue