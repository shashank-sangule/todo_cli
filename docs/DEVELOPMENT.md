# Development Guide

This guide provides detailed instructions for setting up a development environment and contributing to the Rust Todo CLI project.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Performance](#performance)
- [Documentation](#documentation)
- [Debugging](#debugging)
- [Release Process](#release-process)

## Prerequisites

### System Requirements

- **Rust**: 1.70.0 or later (2021 edition)
- **Git**: For version control
- **Editor**: VS Code, IntelliJ, or similar with Rust support
- **Platform**: Linux, macOS, or Windows

### Rust Toolchain Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add required components
rustup component add clippy rustfmt

# Install development tools
cargo install cargo-tarpaulin cargo-audit cargo-outdated cargo-watch
```

### Optional Tools

```bash
# Additional helpful tools
cargo install cargo-edit          # cargo add, cargo rm commands
cargo install cargo-expand        # Show macro expansions
cargo install flamegraph          # Performance profiling
cargo install cargo-bloat         # Analyze binary size
```

## Development Setup

### 1. Clone Repository

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/todo_cli.git
cd todo_cli

# Add upstream remote
git remote add upstream https://github.com/shashank-sangule/todo_cli.git

# Verify remotes
git remote -v
```

### 2. Branch Strategy

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Or bug fix branch
git checkout -b bugfix/issue-description

# Or documentation branch
git checkout -b docs/documentation-update
```

### 3. Initial Build

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Verify everything works
cargo run -- --help
```

### 4. IDE Configuration

#### VS Code Setup

Install extensions:
```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tamasfe.even-better-toml",
    "usernamehw.errorlens",
    "vadimcn.vscode-lldb"
  ]
}
```

Settings (`.vscode/settings.json`):
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "rust-analyzer.rustfmt.extraArgs": ["+nightly"],
  "files.watcherExclude": {
    "**/target/**": true
  }
}
```

#### IntelliJ IDEA/CLion

1. Install Rust plugin
2. Configure rustfmt and clippy integration
3. Set up run configurations for tests and examples

## Project Structure

### Source Code Organization

```
src/
├── cli/                    # Command-line interface layer
│   ├── commands.rs         #   Clap command definitions
│   ├── handlers.rs         #   Command execution logic
│   └── mod.rs              #   Module exports
├── display/                # Presentation layer
│   ├── formatter.rs        #   Data formatting utilities
│   ├── table.rs           #   Table display and rendering
│   └── mod.rs             #   Module exports
├── todo/                   # Core domain logic
│   ├── error.rs           #   Error types and handling
│   ├── filters.rs         #   Query and filtering logic
│   ├── item.rs            #   TodoItem data structure
│   ├── manager.rs         #   Business logic and CRUD operations
│   └── mod.rs             #   Module exports
├── utils/                  # Shared utilities
│   ├── date.rs            #   Date parsing and validation
│   ├── validation.rs      #   Input validation helpers
│   └── mod.rs             #   Module exports
├── lib.rs                 # Library entry point
└── main.rs                # Application entry point
```

### Configuration Files

```
.github/
├── workflows/
│   └── rust-ci.yml        # CI/CD pipeline
├── ISSUE_TEMPLATE/        # Issue templates
└── pull_request_template.md

docs/                      # Documentation
├── ARCHITECTURE.md        # System design
├── USAGE.md              # User guide
├── INSTALLATION.md       # Installation instructions
└── ...

tests/                     # Integration tests
├── integration_tests.rs  # CLI integration tests
├── common/               # Test utilities
│   └── mod.rs
└── ...

Cargo.toml                # Project configuration
Cargo.lock                # Dependency lock file
README.md                 # Project overview
CONTRIBUTING.md           # Contribution guidelines
CHANGELOG.md              # Version history
LICENSE                   # MIT license
```

### Dependencies Architecture

```rust
// Core dependencies
clap = "4.5+"              // CLI parsing
chrono = "0.4+"            // Date handling
serde = "1.0+"             // Serialization
thiserror = "2.0+"         // Error handling

// Development dependencies
tempfile = "3.8"           // Test utilities
assert_cmd = "2.0"         // CLI testing
predicates = "3.0"         // Test assertions
```

## Development Workflow

### Daily Development Cycle

```bash
# 1. Sync with upstream
git fetch upstream
git rebase upstream/main

# 2. Start development server (optional)
cargo watch -x 'run -- list'

# 3. Make changes and test continuously
cargo watch -x test

# 4. Check code quality
cargo clippy
cargo fmt

# 5. Run full test suite
cargo test --all-features
```

### Making Changes

#### 1. Code Changes
```bash
# Edit source files
vim src/todo/item.rs

# Test your changes
cargo test --lib
cargo run -- add "test task"
```

#### 2. Adding Dependencies
```bash
# Add new dependency
cargo add serde_yaml --features derive

# Update existing dependency
cargo update
```

#### 3. Documentation Updates
```bash
# Update API docs
vim src/lib.rs

# Test doc generation
cargo doc --open

# Test documentation examples
cargo test --doc
```

### Commit Guidelines

Follow [Conventional Commits](https://conventionalcommits.org/):

```bash
# Feature commits
git commit -m "feat: add natural language date parsing"
git commit -m "feat(cli): add --sort-by option for list command"

# Bug fix commits  
git commit -m "fix: handle empty todo titles correctly"
git commit -m "fix(date): resolve timezone parsing edge case"

# Documentation commits
git commit -m "docs: update installation guide for Windows"
git commit -m "docs(api): add examples to TodoManager methods"

# Refactoring commits
git commit -m "refactor: extract date parsing to separate module"

# Test commits
git commit -m "test: add integration tests for filtering"
```

## Testing

### Test Organization

```rust
// Unit tests (in each module)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
    }
}

// Integration tests (tests/ directory)
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_cli_integration() {
    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.arg("add").arg("test task");
    cmd.assert().success();
}
```

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# Specific test
cargo test test_add_todo

# With output
cargo test -- --nocapture

# Parallel test execution
cargo test --jobs 4
```

### Test Coverage

```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# View coverage
open tarpaulin-report.html
```

### Writing Effective Tests

#### Unit Test Example
```rust
#[test]
fn test_todo_item_creation() {
    let todo = TodoItem::new(
        1,
        "Test task".to_string(),
        None,
        false,
        None,
        Some(Priority::High),
        None
    );

    assert_eq!(todo.id(), 1);
    assert_eq!(todo.title(), "Test task");
    assert_eq!(todo.priority(), Some(Priority::High));
    assert!(!todo.completed());
}
```

#### Integration Test Example
```rust
#[test]
fn test_add_and_list_workflow() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();

    // Add todo
    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.args(&["--file", file_path, "add", "Test task"]);
    cmd.assert().success();

    // List todos
    let mut cmd = Command::cargo_bin("todo").unwrap();
    cmd.args(&["--file", file_path, "list"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test task"));
}
```

#### Property-Based Testing
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_validate_text_length(input in ".*", max_len in 1..1000usize) {
        let result = validate_text(&input, max_len);

        if input.trim().is_empty() {
            assert!(result.is_err());
        } else if input.trim().len() <= max_len {
            assert!(result.is_ok());
        } else {
            assert!(result.is_err());
        }
    }
}
```

## Code Quality

### Formatting

```bash
# Format all code
cargo fmt

# Check formatting without applying
cargo fmt -- --check

# Format specific file
cargo fmt src/lib.rs
```

### Linting

```bash
# Run clippy
cargo clippy

# Clippy with warnings as errors
cargo clippy -- -D warnings

# Clippy for all targets
cargo clippy --all-targets --all-features
```

### Common Clippy Configurations

Add to `Cargo.toml`:
```toml
[lints.clippy]
# Deny common issues
unwrap_used = "deny"
expect_used = "warn"
panic = "deny"
todo = "warn"

# Allow certain lints if needed
missing_docs_in_private_items = "allow"
```

### Security Auditing

```bash
# Check for security vulnerabilities
cargo audit

# Update audit database
cargo audit fix
```

### Dependency Management

```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Check dependency tree
cargo tree

# Analyze binary size
cargo bloat --release --crates
```

## Performance

### Benchmarking

Create `benches/benchmark.rs`:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use todo_app::TodoManager;

fn benchmark_add_todos(c: &mut Criterion) {
    c.bench_function("add_1000_todos", |b| {
        b.iter(|| {
            let mut manager = create_test_manager();
            for i in 0..1000 {
                manager.add_todo(
                    black_box(format!("Task {}", i)),
                    None, None, None, None
                ).unwrap();
            }
        })
    });
}

criterion_group!(benches, benchmark_add_todos);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

### Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Profile release build
cargo flamegraph --bin todo -- list

# Profile specific test
cargo flamegraph --test integration_tests
```

### Memory Usage Analysis

```bash
# Analyze memory usage with valgrind (Linux)
valgrind --tool=massif cargo run -- list
ms_print massif.out.*

# Check for memory leaks
valgrind --leak-check=full cargo run -- add "test"
```

## Documentation

### API Documentation

```bash
# Generate documentation
cargo doc

# Generate and open documentation
cargo doc --open

# Include private items
cargo doc --document-private-items

# Test documentation examples
cargo test --doc
```

### Writing Good Documentation

```rust
/// Adds a new todo item to the manager
///
/// # Arguments
///
/// * `title` - The todo title (max 140 characters)
/// * `description` - Optional detailed description
/// * `due_date` - Optional due date in various formats
/// * `priority` - Optional priority level
/// * `tags` - Optional list of tags
///
/// # Returns
///
/// Returns `Ok(())` if the todo was added successfully, or a `TodoError`
/// if validation fails or the operation cannot be completed.
///
/// # Examples
///
/// ```rust
/// use todo_app::TodoManager;
/// 
/// let mut manager = TodoManager::new("todos.json".to_string())?;
/// manager.add_todo(
///     "Buy groceries".to_string(),
///     Some("Milk, eggs, bread".to_string()),
///     Some("tomorrow"),
///     Some("medium"),
///     Some(vec!["shopping".to_string()])
/// )?;
/// # Ok::<(), todo_app::TodoError>(())
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The title is empty or too long
/// - The due date format is invalid
/// - The priority is not one of "high", "medium", "low"
/// - File I/O operations fail
pub fn add_todo(
    &mut self,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    priority: Option<&str>,
    tags: Option<Vec<String>>,
) -> TodoResult<()> {
    // Implementation
}
```

## Debugging

### Debug Logging

Add to your code:
```rust
use log::{debug, info, warn, error};

pub fn add_todo(&mut self, title: String) -> TodoResult<()> {
    debug!("Adding todo with title: {}", title);

    let validated_title = validate_text(&title, 140)?;
    info!("Title validated successfully");

    // ... rest of implementation
}
```

Run with logging:
```bash
RUST_LOG=debug cargo run -- add "test task"
RUST_LOG=todo_app=trace cargo run -- list
```

### Debugger Setup

#### VS Code with CodeLLDB
1. Install CodeLLDB extension
2. Create `.vscode/launch.json`:
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug todo",
            "cargo": {
                "args": ["build", "--bin=todo"],
                "filter": {
                    "name": "todo",
                    "kind": "bin"
                }
            },
            "args": ["add", "debug task"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

#### Command Line Debugging
```bash
# Build with debug info
cargo build

# Debug with gdb
gdb target/debug/todo
(gdb) run add "test task"
(gdb) bt  # backtrace on error
```

### Testing in Isolation

Create isolated test environment:
```bash
# Create temporary directory
mkdir /tmp/todo-test
cd /tmp/todo-test

# Test specific functionality
todo --file test.json add "isolated test"
todo --file test.json list
```

## Release Process

### Version Management

1. **Update version** in `Cargo.toml`:
```toml
[package]
version = "0.2.0"
```

2. **Update CHANGELOG.md**:
```markdown
## [0.2.0] - 2025-08-15

### Added
- Natural language date parsing
- Enhanced filtering options

### Fixed
- Date validation edge cases
```

3. **Create git tag**:
```bash
git commit -m "chore: bump version to 0.2.0"
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```

### Pre-release Checklist

```bash
# 1. Full test suite
cargo test --all-features

# 2. Code quality checks  
cargo clippy -- -D warnings
cargo fmt -- --check

# 3. Security audit
cargo audit

# 4. Documentation
cargo doc --document-private-items
cargo test --doc

# 5. Integration tests
cargo test --test integration_tests

# 6. Performance regression test
cargo bench

# 7. Cross-platform build test
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
```

### Publishing to Crates.io

```bash
# Dry run
cargo publish --dry-run

# Actual publish
cargo publish
```

## Continuous Integration

The project uses GitHub Actions for CI/CD. The workflow:

1. **Code Quality**: Format, lint, and audit checks
2. **Testing**: Unit, integration, and documentation tests
3. **Cross-Platform**: Test on Linux, macOS, and Windows
4. **Performance**: Benchmark regression detection
5. **Security**: Dependency vulnerability scanning

Local CI simulation:
```bash
# Run the same checks as CI
./scripts/ci-check.sh
```

Create `scripts/ci-check.sh`:
```bash
#!/bin/bash
set -e

echo "🔍 Running code quality checks..."
cargo fmt --check
cargo clippy -- -D warnings

echo "🧪 Running tests..."
cargo test --all-features

echo "🔒 Running security audit..."
cargo audit

echo "📚 Testing documentation..."
cargo doc --document-private-items
cargo test --doc

echo "✅ All checks passed!"
```

---

## Quick Reference Commands

### Daily Development
```bash
cargo watch -x 'test'          # Continuous testing
cargo watch -x 'run -- list'   # Continuous running
cargo check                    # Fast syntax check
cargo test --lib              # Unit tests only
```

### Code Quality
```bash
cargo fmt && cargo clippy && cargo test  # Quick quality check
cargo tarpaulin --out Html               # Coverage report
cargo audit                              # Security check
```

### Documentation
```bash
cargo doc --open               # Generate and view docs
cargo test --doc              # Test doc examples
```

### Debugging
```bash
RUST_LOG=debug cargo run       # Debug logging
RUST_BACKTRACE=1 cargo test    # Full backtraces
```

**For more information, see [CONTRIBUTING.md](../CONTRIBUTING.md) and [ARCHITECTURE.md](ARCHITECTURE.md).**
