# Justfile Guide for AI Assistants

## Quick Reference

**Command runner, not build system.** Recipes execute shell commands with automatic error stopping.

## Core Recipe Patterns

### Basic Recipe
```justfile
task:
    command1
    command2  # Only runs if command1 succeeds
```

### Recipe with Parameters
```justfile
build target:
    cargo build --bin {{target}}

# With defaults
test target='all':
    cargo test {{target}}

# Variadic
backup +files:
    cp {{files}} /backup/
```

## Shell Execution (Critical)

### Default: Linewise Execution
Each line runs separately in `sh -cu`. No state carries between lines:
```justfile
bad-example:
    cd /tmp        # This directory change is lost
    echo $(pwd)    # Still in original directory
```

### Shebang: Script Execution  
Entire recipe runs as one script. State persists:
```justfile
#!/usr/bin/env bash
good-example:
    cd /tmp
    echo $(pwd)    # Now in /tmp
```

**When to use shebangs:**
- Multi-line bash scripts
- Variables/state between commands
- Complex conditionals
- `set -e` error handling

**Don't need shebangs for:**
- Simple command sequences (just handles errors automatically)
- Single-line commands with substitution

## Variables

```justfile
# Assignment
version := "1.0.0"
git_hash := `git rev-parse HEAD`
home := env_var_or_default('HOME', '/tmp')

# Usage
deploy:
    ./deploy.sh --version {{version}}
```

## Dependencies

```justfile
# Basic: test runs before build
build: test
    cargo build

# Multiple: both clean and test run first
build: clean test
    cargo build

# Subsequent: docs runs after build  
build: test && docs
    cargo build
```

## Error Handling

**Default behavior:** Recipes stop on first command failure.

```justfile
# Continue on error
test-all:
    -cargo test     # - prefix continues on failure
    echo "Done"

# Silent commands
quiet:
    @echo "No command echo"  # @ prefix hides command
```

## Conditionals

```justfile
# Conditional assignment
flag := if env_var_or_default('DEBUG', 'false') == 'true' { '--debug' } else { '' }

# Platform-specific
install:
    {{if os_family() == "unix" { "apt install" } else { "choco install" }}} package
```

## Built-in Functions

**Environment:**
- `env_var('KEY')`, `env_var_or_default('KEY', 'default')`

**System:**
- `os()` - "linux", "macos", "windows"
- `os_family()` - "unix", "windows" 
- `arch()` - "x86_64", "aarch64", etc.

**Paths:**
- `justfile_directory()` - Directory containing justfile
- `join('path', 'parts')` - Join path components

**Strings:**
- `lowercase()`, `uppercase()`, `trim()`
- `replace('old', 'new')`

## Command Line

```bash
just                  # Run default recipe
just recipe-name      # Run specific recipe
just --list          # Show all recipes
just --show recipe   # Show recipe without running
just --dry-run       # Show commands without executing
```

## Common Patterns

### Default Recipe
```justfile
# First recipe is default
default: test build

help:
    @just --list
```

### Cross-Platform
```justfile
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

install:
    {{if os_family() == "windows" { "choco install" } else { "brew install" }}} tool
```

### Environment Setup
```justfile
set dotenv-load      # Load .env files
set export          # Export variables to environment

setup:
    git submodule update --init
    npm install
```

### Development Workflow
```justfile
dev: setup
    cargo watch -x check

test: 
    cargo test

build: test
    cargo build --release

deploy: build
    ./deploy.sh
```

## Recipe Attributes

```justfile
[private]           # Hide from --list
[confirm]          # Require confirmation
[linux]            # Platform-specific
[no-cd]            # Don't change to justfile directory

[confirm]
[private]
dangerous-operation:
    rm -rf /tmp/*
```

## Key Gotchas

1. **Shell state doesn't persist between lines** unless using shebangs
2. **Command substitution `$(...)` works in regular recipes** - no shebang needed
3. **Recipes stop on first failure** - use `-` prefix to continue
4. **Variables are immutable** - use `{{}}` for substitution
5. **No spaces around `:=`** in assignments
6. **Dependencies run only once** per invocation

## When You Need Shebangs

```justfile
# Need shebang - bash conditionals
#!/usr/bin/env bash
conditional-deploy:
    if [[ "$ENV" == "prod" ]]; then
        ./deploy-prod.sh
    else
        ./deploy-dev.sh
    fi

# Don't need shebang - simple commands
simple-deploy:
    ./deploy.sh
    ./notify.sh
```