# Contributing to Rust Todo CLI

Thank you for your interest in contributing to Rust Todo CLI! This guide will help you get started with contributing to this project.

## 📋 Quick Start

### Prerequisites
- Rust 1.70.0 or later
- Git
- GitHub account

### Setup
1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/todo_cli.git
   cd todo_cli
   ```
3. **Add upstream** remote:
   ```bash
   git remote add upstream https://github.com/shashank-sangule/todo_cli.git
   ```

## 🎯 Development Workflow

### Branch Strategy
- `main` - Stable releases
- `develop` - Integration branch for features
- `feature/*` - New features
- `bugfix/*` - Bug fixes
- `docs/*` - Documentation updates

### Making Changes
1. **Create a branch** from main:
   ```bash
   git checkout main
   git pull upstream main
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our coding standards

3. **Test thoroughly**:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

4. **Commit with conventional commits**:
   ```bash
   git commit -m "feat: add new filtering option for tags"
   ```

5. **Push and create PR**:
   ```bash
   git push origin feature/your-feature-name
   ```

## 📝 Coding Standards

### Code Style
- Follow **Rust 2021 edition** conventions
- Use `cargo fmt` for consistent formatting
- Ensure `cargo clippy -- -D warnings` passes
- Write idiomatic Rust code

### Documentation
- **Document all public APIs** with `///` comments
- Include **practical examples** in doc comments
- Update README.md for user-facing changes
- Add inline comments for complex logic

### Error Handling
- Use `Result<T, TodoError>` for fallible operations
- Provide **context-rich error messages**
- Don't use `unwrap()` or `panic!()` in library code
- Chain errors using `?` operator when appropriate

### Testing
- **Unit tests** for all new functions
- **Integration tests** for CLI workflows
- **Property tests** for complex validation logic
- **Error scenario tests** for edge cases
- Maintain **>90% test coverage**

### Performance
- Avoid unnecessary allocations
- Use iterator chains over explicit loops
- Profile performance-critical paths
- Document Big O complexity for algorithms

## 🧪 Testing Guidelines

### Running Tests
```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests
cargo test --test integration_tests

# With coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_feature_success() {
        // Arrange
        let input = "test input";

        // Act
        let result = function_under_test(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_output);
    }

    #[test]
    fn test_feature_error_case() {
        let result = function_under_test("invalid input");
        assert!(matches!(result.unwrap_err(), TodoError::InvalidInput { .. }));
    }
}
```

## 🚀 Pull Request Process

### Before Submitting
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (if applicable)
- [ ] Self-review completed

### PR Template
When creating a PR, include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change)
- [ ] New feature (non-breaking change)
- [ ] Breaking change (fix or feature that causes existing functionality to change)
- [ ] Documentation update

## How Has This Been Tested?
- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual testing

## Checklist
- [ ] My code follows the project's style guidelines
- [ ] I have performed a self-review
- [ ] I have commented my code where necessary
- [ ] My changes generate no new warnings
- [ ] New and existing tests pass
```

### Review Process
1. **Automated checks** must pass
2. **Code review** by maintainers
3. **Discussion** and feedback incorporation
4. **Final approval** and merge

## 🎨 Types of Contributions

### Bug Fixes
- Check existing issues first
- Create issue if none exists
- Include reproduction steps
- Add regression tests

### New Features
- **Discuss first** via GitHub issue
- Follow existing patterns
- Include comprehensive tests
- Update documentation

### Documentation
- Fix typos and unclear sections
- Add examples and use cases
- Improve API documentation
- Update installation guides

### Performance Improvements
- **Benchmark before and after**
- Include performance justification
- Ensure no functionality regression
- Document performance characteristics

## 🐛 Reporting Issues

### Bug Reports
Use the bug report template:
```markdown
## Bug Description
Clear description of the bug

## Steps to Reproduce
1. Run command: `todo add "test"`
2. Observe behavior

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.70.0]
- Todo CLI version: [e.g., 0.1.0]
```

### Feature Requests
```markdown
## Feature Description
Clear description of the proposed feature

## Motivation
Why is this feature needed?

## Proposed Solution
How should this feature work?

## Alternatives Considered
What other approaches were considered?
```

## 🏗️ Development Environment

### Recommended Tools
- **IDE**: VS Code with Rust Analyzer
- **Debugger**: Built-in Rust debugger or GDB
- **Profiler**: `cargo flamegraph` for performance analysis

### Useful Commands
```bash
# Fast development cycle
cargo check                    # Quick syntax check
cargo test --lib              # Unit tests only
cargo clippy --fix            # Auto-fix clippy suggestions

# Documentation
cargo doc --open              # Generate and view docs
cargo doc --document-private-items  # Include private items

# Performance
cargo build --release         # Optimized build
cargo bench                   # Run benchmarks

# Maintenance
cargo audit                   # Security vulnerabilities
cargo outdated               # Check for updates
cargo tree                   # Dependency tree
```

### Environment Variables
```bash
# Enable debug logging
export RUST_LOG=debug

# Backtrace on panic
export RUST_BACKTRACE=1

# Show all clippy lints
export RUSTFLAGS="-W clippy::pedantic"
```

## 📋 Code Review Checklist

### For Reviewers
- [ ] **Functionality**: Does the code work as intended?
- [ ] **Tests**: Are there adequate tests covering the changes?
- [ ] **Performance**: Are there any performance concerns?
- [ ] **Security**: Are there any security implications?
- [ ] **Documentation**: Is documentation updated and clear?
- [ ] **Style**: Does the code follow project conventions?
- [ ] **Backwards Compatibility**: Does this break existing functionality?

### For Contributors
- [ ] **Self-review**: Have you reviewed your own code?
- [ ] **Edge cases**: Have you considered error conditions?
- [ ] **Testing**: Have you tested both success and failure paths?
- [ ] **Documentation**: Are public APIs documented?
- [ ] **Dependencies**: Are new dependencies justified?

## 🏆 Recognition

### Contributors
All contributors will be recognized in:
- Repository contributors section
- Release notes for significant contributions
- Special mentions for first-time contributors

### Types of Recognition
- **Code Contributors**: Direct code improvements
- **Documentation Contributors**: Docs, examples, guides
- **Community Contributors**: Issue reporting, discussions
- **Infrastructure Contributors**: CI/CD, tooling improvements

## 📚 Resources

### Rust Learning
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Project-Specific
- [Architecture Documentation](docs/ARCHITECTURE.md)
- [Development Setup](docs/DEVELOPMENT.md)
- [API Documentation](https://docs.rs/todo_app)

### Tools and Crates
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [Cargo Commands](https://doc.rust-lang.org/cargo/commands/)
- [Testing Patterns](https://doc.rust-lang.org/book/ch11-00-testing.html)

## 🤝 Community Guidelines

### Code of Conduct
- **Be respectful** and inclusive
- **Provide constructive feedback**
- **Help newcomers** learn and contribute
- **Focus on the code**, not the person
- **Assume positive intent**

### Communication
- Use **GitHub issues** for bugs and features
- Use **PR discussions** for code-specific questions
- Be **patient** with review process
- **Ask questions** if anything is unclear

## 📞 Getting Help

### Where to Get Help
- **GitHub Issues**: Technical questions and bugs
- **GitHub Discussions**: General questions and ideas
- **Email**: [your.sanguleshashank@gmail.com](mailto:your.sanguleshashank@gmail.com)

### Response Times
- **Issues**: Within 48 hours
- **PRs**: Within 72 hours
- **Security issues**: Within 24 hours

---

**Thank you for contributing to Rust Todo CLI! 🦀✨**

> Every contribution, no matter how small, helps make this project better for everyone.
