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
    #!/bin/bash
    {{rust_flags}} cargo check
    if ! {{rust_flags}} cargo test; then
        just test-summary
        exit 1
    fi
    just test-summary
    cargo clippy -- -D warnings
    cargo fmt
    

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

# Analyze stack usage per function  
stack-analysis:
    #!/bin/bash
    if rustup toolchain list | grep -q nightly; then
        RUSTFLAGS="-Z print-stack-usage" cargo +nightly build --release 2>/dev/null || echo "Nightly analysis failed"
        find target/release/build -name "*.su" -exec cat {} \; 2>/dev/null | sort -k2 -nr > stack_usage_report.txt || echo "No .su files found"
        echo "Stack usage report generated in stack_usage_report.txt"
    else
        echo "Nightly toolchain not available for stack analysis"
    fi

# Generate comprehensive stack analysis (Mac-compatible)
stack-report:
    #!/bin/bash
    echo "Generating comprehensive stack analysis for Mac..."
    
    # Ensure we have a release build
    cargo build --release
    
    # Method 1: Extract function information with otool and calculate sizes
    if [ -f target/release/rustleaf ]; then
        echo "Analyzing function sizes with otool..."
        
        # Get all function symbols and their addresses
        otool -tv target/release/rustleaf | grep -E '^[A-Za-z_]' | \
        awk '{print $1}' | sed 's/:$//' > function_names.txt
        
        # Get symbol table with addresses
        nm target/release/rustleaf | grep -E ' [Tt] ' | \
        grep -E '_[A-Za-z]' | sort > symbol_addresses.txt
        
        # Generate detailed otool analysis
        otool -tv target/release/rustleaf > otool_full_analysis.txt
        echo "Function analysis saved to otool_full_analysis.txt"
    fi
    
    # Method 2: Use bloaty if available (better for size analysis)
    if command -v bloaty >/dev/null 2>&1; then
        echo "Running bloaty analysis..."
        bloaty target/release/rustleaf -d symbols | head -50 > bloaty_analysis.txt
        echo "Bloaty analysis saved to bloaty_analysis.txt"
    else
        echo "Install bloaty for detailed size analysis: brew install bloaty"
    fi
    
    # Method 3: LLVM-based analysis
    echo "Generating LLVM IR for analysis..."
    cargo rustc --release --bin rustleaf -- --emit=llvm-ir -C opt-level=3 -o target/release/rustleaf.ll 2>/dev/null || true
    if [ -f target/release/rustleaf.ll ]; then
        echo "Extracting function information from LLVM IR..."
        grep -E "define.*@" target/release/rustleaf.ll | \
        sed 's/define [^@]*@//g' | sed 's/(.*//g' | \
        head -30 > llvm_functions.txt
        echo "LLVM function list saved to llvm_functions.txt"
    fi
    
    # Method 4: Size analysis with size command
    echo "Basic size analysis..."
    size target/release/rustleaf > size_analysis.txt
    echo "Size analysis saved to size_analysis.txt"
    
    echo ""
    echo "Mac stack analysis complete. Generated files:"
    [ -f otool_full_analysis.txt ] && echo "  ✓ otool_full_analysis.txt (function disassembly)"
    [ -f symbol_addresses.txt ] && echo "  ✓ symbol_addresses.txt (symbol table)"
    [ -f bloaty_analysis.txt ] && echo "  ✓ bloaty_analysis.txt (detailed size analysis)"
    [ -f llvm_functions.txt ] && echo "  ✓ llvm_functions.txt (LLVM function list)"
    [ -f size_analysis.txt ] && echo "  ✓ size_analysis.txt (basic size info)"
    echo ""
    echo "For the most accurate results, install bloaty: brew install bloaty"

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
