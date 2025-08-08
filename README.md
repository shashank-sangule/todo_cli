# 📋 Rust Todo CLI

[![Build Status](https://github.com/shashank-sangule/todo_cli/workflows/Rust%20CI/badge.svg)](https://github.com/shashank-sangule/todo_cli/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

> A blazingly fast, feature-rich command-line todo application built with Rust, demonstrating systems programming expertise and clean architecture principles.

## 🌟 Features

### Core Functionality
- **📝 Task Management**: Add, edit, delete, and toggle todo items with rich metadata
- **🎯 Smart Filtering**: Filter by priority, status, due dates, and custom time ranges
- **📊 Flexible Sorting**: Sort by due date, priority, or combined criteria with ascending/descending order
- **🏷️ Tag System**: Organize tasks with multiple tags for better categorization
- **📅 Natural Date Parsing**: Support for "tomorrow", "next week", "2025-12-25", and more formats

### Technical Highlights
- **🦀 Memory Safe**: Written in Rust with zero unsafe code and comprehensive error handling
- **⚡ High Performance**: Optimized algorithms with O(n log n) sorting and efficient data structures
- **🛡️ Robust Error Handling**: 12 distinct error types with context-rich, user-friendly messages
- **🧪 Well Tested**: 90%+ test coverage with 50+ unit and integration tests
- **🔧 Extensible Architecture**: Clean separation of concerns with modular, layered design

## 🚀 Quick Start

### Installation

#### From Source (Recommended for development)
```bash
git clone https://github.com/shashank-sangule/todo_cli.git
cd todo_cli
cargo build --release
cargo install --path .
```

#### Prerequisites
- Rust 1.70.0 or later
- Cargo (comes with Rust)

### Basic Usage

```bash
# Add a simple todo
todo add "Buy groceries"

# Add a todo with all options
todo add "Complete project" \
  --priority high \
  --due-date "2025-08-15 14:00" \
  --description "Finish the Rust todo app" \
  --tags "work,urgent,deadline"

# List all todos
todo list
```

**Output:**
```
ID  ✓ Todo                              Due Date                       Priority
─────────────────────────────────────────────────────────────────────────────────
1   ⬜ Buy groceries                     -                              -
2   ⬜ Complete project                  🔴 15-08-2025 14:00 (overdue)  🔴 High
```

### Advanced Usage

```bash
# Advanced filtering and sorting
todo list --priority high --sort-by due+priority --asc
todo list --overdue --only-pending
todo list --due-within 7 --sort-by priority

# Task management
todo toggle 1      # Mark as complete/incomplete
todo edit 1 --priority low --description "Updated description"
todo delete 1      # Remove todo
todo clear-list    # Remove all todos

# Custom file location
todo --file work-todos.json add "Team meeting"
```

## 📚 Documentation

### User Guide
- [Installation Guide](docs/INSTALLATION.md) - Detailed setup instructions
- [Usage Examples](docs/USAGE.md) - Comprehensive command examples
- [Date Format Reference](docs/DATE_FORMATS.md) - Supported date formats
- [Filtering and Sorting](docs/FILTERING.md) - Advanced query capabilities

### Developer Resources
- [Architecture Overview](docs/ARCHITECTURE.md) - System design and patterns
- [API Documentation](https://docs.rs/todo_app) - Generated API docs
- [Contributing Guide](CONTRIBUTING.md) - Development guidelines
- [Development Setup](docs/DEVELOPMENT.md) - Local development guide

## 🏗️ Architecture

This project demonstrates several software engineering best practices:

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   CLI Layer     │    │   Domain Layer   │    │  Infrastructure │
│                 │    │                  │    │                 │
│ • Commands      │◄──►│ • TodoManager    │◄──►│ • File I/O      │
│ • Argument      │    │ • TodoItem       │    │ • Serialization │
│   Parsing       │    │ • Business Logic │    │ • Validation    │
│ • Display       │    │ • Error Handling │    │ • Date Parsing  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Key Design Decisions

- **Command Pattern**: Extensible command handling with clear separation of concerns
- **Repository Pattern**: Abstracted data persistence for easy testing and future database integration
- **Rich Error Types**: Comprehensive error handling with `thiserror` for context-rich error messages
- **Type Safety**: Extensive use of Rust's type system with enums for priorities and sort criteria
- **Functional Patterns**: Iterator chains and immutable operations for data processing

## 📊 Performance Characteristics

| Operation | Complexity | Performance Notes |
|-----------|------------|-------------------|
| Add Todo | O(1) | Constant time insertion with validation |
| Find by ID | O(n) | Linear search (optimizable to O(1) with indexing) |
| Filter | O(n) | Single pass with efficient iterator chains |
| Sort | O(n log n) | Comparison-based sorting with multiple criteria |
| Save/Load | O(n) | JSON serialization with atomic file operations |

**Memory Usage**: ~1-2MB runtime footprint with efficient string handling and minimal allocations.

## 🧪 Testing

This project maintains high code quality through comprehensive testing:

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Run specific test categories
cargo test --lib           # Unit tests only
cargo test --test '*'      # Integration tests
cargo test --doc           # Documentation tests

# Run with verbose output
cargo test -- --nocapture
```

### Test Coverage Breakdown
- **Unit Tests**: Core business logic, utilities, and error handling (45+ tests)
- **Integration Tests**: Complete CLI workflows and edge cases (8+ tests)
- **Documentation Tests**: All code examples in documentation compile and run
- **Property Tests**: Input validation and invariant checking

**Current Coverage**: 90%+ with comprehensive error scenario testing.

## 🛠️ Development

### Quick Development Setup
```bash
# Clone and setup
git clone https://github.com/shashank-sangule/todo_cli.git
cd todo_cli

# Install development tools
rustup component add clippy rustfmt
cargo install cargo-audit cargo-outdated

# Development workflow
cargo check          # Fast syntax checking
cargo test            # Run test suite
cargo clippy -- -D warnings  # Linting with warnings as errors
cargo fmt             # Code formatting
cargo audit           # Security vulnerability check
```

### Project Structure
```
src/
├── cli/              # Command-line interface layer
│   ├── commands.rs   #   Command definitions with clap
│   ├── handlers.rs   #   Command execution logic
│   └── mod.rs        #   Module exports
├── display/          # Presentation layer
│   ├── formatter.rs  #   Data formatting utilities
│   ├── table.rs      #   Table rendering and display
│   └── mod.rs        #   Module exports
├── todo/             # Core domain logic
│   ├── error.rs      #   Custom error types and handling
│   ├── filters.rs    #   Query and filtering logic
│   ├── item.rs       #   TodoItem data structure
│   ├── manager.rs    #   Business logic and CRUD operations
│   └── mod.rs        #   Module exports
├── utils/            # Shared utilities
│   ├── date.rs       #   Date parsing and validation
│   ├── validation.rs #   Input validation helpers
│   └── mod.rs        #   Module exports
├── lib.rs            # Library entry point and exports
└── main.rs           # Application entry point
```

## 🤝 Contributing

Contributions are welcome! This project follows standard Rust conventions and welcomes developers of all skill levels.

### Quick Contributing Guide
1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Write** tests for your changes
4. **Ensure** all tests pass (`cargo test`)
5. **Run** quality checks (`cargo clippy`, `cargo fmt`)
6. **Submit** a pull request

For detailed guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

### Development Commands
```bash
# Quality checks (run before committing)
cargo test && cargo clippy -- -D warnings && cargo fmt --check

# Generate documentation
cargo doc --open --document-private-items

# Performance benchmarks
cargo bench
```

## 📈 Roadmap

### Planned Features (v0.2.0)
- [ ] **Database Backend**: SQLite support for improved performance with large datasets
- [ ] **Recurring Tasks**: Support for daily, weekly, monthly recurring todos
- [ ] **Task Dependencies**: Link todos with prerequisite relationships
- [ ] **Time Tracking**: Built-in time logging for completed tasks
- [ ] **Import/Export**: Support for CSV, JSON, and other todo formats

### Performance Improvements
- [ ] **Indexed Lookups**: HashMap-based O(1) ID operations for large todo lists
- [ ] **Lazy Loading**: Paginated loading for datasets with 1000+ items
- [ ] **Smart Caching**: Cache frequently accessed filtered/sorted views
- [ ] **Incremental Saves**: Only persist changed data to reduce I/O

### Advanced Features (v0.3.0+)
- [ ] **Web Interface**: Optional REST API with lightweight web UI
- [ ] **Multi-Device Sync**: Cloud synchronization across devices
- [ ] **Plugin System**: Extensible architecture for custom commands
- [ ] **Natural Language**: Enhanced date parsing with more expressions

## 🏆 Technical Achievements

This project showcases several advanced Rust concepts and practices:

### Systems Programming Excellence
- **Zero-Cost Abstractions**: High-level APIs with no runtime performance penalty
- **Memory Safety**: Complete memory safety without garbage collection
- **Ownership System**: Efficient resource management with borrowing and lifetimes
- **Concurrency Safety**: Thread-safe operations where applicable

### Software Engineering Practices
- **Error Handling**: Robust error propagation with structured error types
- **Testing Strategy**: Comprehensive test pyramid with unit, integration, and property tests
- **Documentation**: Extensive inline documentation with practical examples
- **CI/CD Pipeline**: Automated testing, linting, security audits, and formatting checks

### Rust Ecosystem Integration
- **Crate Selection**: Thoughtful dependency choices with established, maintained crates
- **API Design**: Follows Rust API guidelines for consistency and ergonomics
- **Performance**: Optimized for both compile-time and runtime efficiency
- **Cross-Platform**: Supports Linux, macOS, and Windows out of the box

## 📋 System Requirements

### Runtime Requirements
- **Operating System**: Linux, macOS, or Windows
- **Architecture**: x86_64, ARM64 (Apple Silicon supported)
- **Memory**: Minimal RAM usage (~1-2MB for typical workloads)
- **Storage**: JSON file-based persistence (human-readable format)

### Development Requirements
- **Rust**: 1.70.0+ (2021 edition)
- **Cargo**: Latest stable version
- **Platform**: Any platform supported by Rust toolchain

### Optional Tools
- **cargo-tarpaulin**: Test coverage reports
- **cargo-audit**: Security vulnerability scanning
- **cargo-outdated**: Dependency update checking

## 🐛 Known Issues & Limitations

### Current Limitations
- **File Locking**: No concurrent access protection for shared todo files
- **Large Datasets**: All todos loaded into memory (optimization planned for v0.2.0)
- **Date Timezone**: Uses local system timezone (UTC support planned)

### Reporting Issues
Found a bug or have a feature request? Please check [existing issues](https://github.com/shashank-sangule/todo_cli/issues) first, then create a new issue with:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- System information (OS, Rust version)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### License Summary
- ✅ Commercial use allowed
- ✅ Modification allowed  
- ✅ Distribution allowed
- ✅ Private use allowed
- ❗ License and copyright notice required

## 🙏 Acknowledgments

Special thanks to the Rust community and the maintainers of these excellent crates:

- [**clap**](https://crates.io/crates/clap) - Powerful command line argument parsing with derive macros
- [**chrono**](https://crates.io/crates/chrono) - Comprehensive date and time handling library
- [**chrono-english**](https://crates.io/crates/chrono-english) - Natural language date parsing ("tomorrow", "next week")
- [**serde**](https://crates.io/crates/serde) - High-performance serialization framework
- [**thiserror**](https://crates.io/crates/thiserror) - Ergonomic error handling with derive macros
- [**tempfile**](https://crates.io/crates/tempfile) - Secure temporary file creation for testing

### Inspiration
This project was inspired by the need for a fast, reliable command-line todo manager that demonstrates modern Rust development practices while solving real productivity challenges.

---

**Made with ❤️ and 🦀 by [Shashank Sangule](https://github.com/shashank-sangule)**

> *This project demonstrates production-ready Rust development practices including clean architecture, comprehensive testing, robust error handling, and professional documentation standards.*

## 📞 Contact & Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/shashank-sangule/todo_cli/issues)
- **Discussions**: [Community discussions and questions](https://github.com/shashank-sangule/todo_cli/discussions)
- **Email**: [your.sanguleshashank@gmail.com](mailto:sanguleshashank@gmail.com)

### Star History

If this project helped you, please consider giving it a ⭐! It helps other developers discover the project and motivates continued development.

```bash
# Quick star from command line (requires GitHub CLI)
gh repo view shashank-sangule/todo_cli --web
```

**Thank you for checking out Rust Todo CLI! 🚀✨**
