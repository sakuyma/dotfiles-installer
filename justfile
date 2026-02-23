# Dotfiles Installer Justfile
# Run commands with: just <command>
# List all commands: just --list

set shell := ["bash", "-c"]
set dotenv-load := true

# Default command (show help)
default:
    @just --list

# ============================================================================
# BUILD & COMPILE
# ============================================================================

# Build the project
build:
    @echo "Building project..."
    cargo build

# Build in release mode (optimized)
build-release:
    @echo "Building release version..."
    cargo build --release

# Check compilation without building
check:
    @echo "Checking compilation..."
    cargo check

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean

# ============================================================================
# TESTING
# ============================================================================

# Run all tests
test:
    @echo "Running tests..."
    cargo test

# Run tests with verbose output
test-verbose:
    @echo "Running tests (verbose)..."
    cargo test -- --nocapture

# Run tests for specific module
test-hardware:
    @echo "Testing hardware module..."
    cargo test hardware --

test-packages:
    @echo "Testing packages module..."
    cargo test packages --

test-configs:
    @echo "Testing configs module..."
    cargo test configs --

# ============================================================================
# LINTING & FORMATTING
# ============================================================================

# Check code style with clippy
lint:
    @echo "Checking code with clippy..."
    cargo clippy -- -D warnings

# Format code with rustfmt
fmt:
    @echo "Formatting code..."
    cargo fmt

# Check if code is formatted
fmt-check:
    @echo "Checking code format..."
    cargo fmt -- --check

# ============================================================================
# DOCUMENTATION
# ============================================================================

# Generate and open documentation
doc:
    @echo "Generating documentation..."
    cargo doc --open

# Generate documentation without opening
doc-gen:
    @echo "Generating documentation..."
    cargo doc --no-deps

# ============================================================================
# RUNNING
# ============================================================================

# Run the application
run:
    @echo "Running dotfiles installer..."
    cargo run

# Run with arguments
run-args *ARGS:
    @echo "Running: {{ARGS}}"
    cargo run -- {{ARGS}}

# Run in debug mode with logging
run-debug:
    @echo "Running in debug mode..."
    RUST_BACKTRACE=1 cargo run

# ============================================================================
# GIT COMMANDS
# ============================================================================

# Check git status
status:
    @echo "Git status:"
    git status

# View recent commits
log:
    @echo "Recent commits:"
    git log --oneline -10

# add all changes and commit
commit message:
    @echo "committing: {{message}}"
    git add .
    git commit -m "{{message}}"

# Push to remote
push:
    @echo "Pushing to remote..."
    git push

# Pull from remote
pull:
    @echo "Pulling from remote..."
    git pull

# ============================================================================
# DEVELOPMENT WORKFLOW
# ============================================================================

# Full development check (format, lint, test, build)
dev-check:
    @echo "Running full development checks..."
    just fmt
    just lint
    just test
    just build
    @echo "All checks passed!"

# Quick check (just lint and test)
quick-check:
    @echo "Running quick checks..."
    just lint
    just test

# Fix common issues (format + clippy fixes)
fix:
    @echo "Fixing issues..."
    cargo fmt
    cargo clippy --fix --allow-dirty

# ============================================================================
# DEBUGGING
# ============================================================================

# Check for compilation errors
diagnose:
    @echo "Diagnosing issues..."
    cargo check 2>&1 | head -20
    @echo ""
    @echo "Run 'cargo check' for full output"

# Update dependencies
update-deps:
    @echo "Updating dependencies..."
    cargo update

# Show dependency tree
deps:
    @echo "Dependency tree:"
    cargo tree

# ============================================================================
# CLEANUP & MAINTENANCE
# ============================================================================

# Remove all build artifacts and cache
purge: clean
    @echo "Purging everything..."
    cargo clean
    rm -rf target/
    @echo "Purged!"

# Initialize new git branch for feature
new-feature name:
    @echo "Creating feature branch: {{name}}"
    git checkout -b feature/{{name}}

# Initialize new bug fix branch
new-bugfix name:
    @echo "Creating bugfix branch: {{name}}"
    git checkout -b bugfix/{{name}}

# ============================================================================
# INSTALLATION & SETUP
# ============================================================================

# Install pre-commit hooks (optional)
setup-hooks:
    @echo "Setting up git hooks..."
    @echo "pre-commit not configured, add manually if needed"

# Check Rust version
rust-version:
    @echo "Rust version:"
    rustc --version
    cargo --version

# ============================================================================
# HELP & INFO
# ============================================================================

# Show project info
info:
    @echo "Project Information:"
    @echo "  Project: Dotfiles Installer"
    @echo "  Language: Rust"
    @echo ""
    @echo "Build Info:"
    cargo --version
    rustc --version
    @echo ""
    @echo "Run 'just --list' to see all available commands"
count:
    @echo "Line count: "
    @find src -name "*.rs" -exec wc -l {} + | tail -1
    @echo ""
    @echo "By module: "
    @find src -name "*.rs" -exec wc -l {} + | sort -rn

# Show help
help:
    @just --list
