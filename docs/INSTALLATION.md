# Installation Guide

This guide provides detailed installation instructions for the Rust Todo CLI application across different platforms and installation methods.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation Methods](#installation-methods)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)
- [Uninstallation](#uninstallation)
- [Development Setup](#development-setup)

## Prerequisites

### System Requirements

#### Minimum Requirements
- **Operating System**: Linux, macOS, or Windows
- **Architecture**: x86_64 or ARM64 (Apple Silicon supported)
- **Memory**: 50MB available RAM
- **Storage**: 10MB available disk space

#### For Development
- **Rust**: 1.70.0 or later
- **Cargo**: Latest stable version (comes with Rust)
- **Git**: For source code management
- **Internet Connection**: For downloading dependencies

### Rust Installation

If you don't have Rust installed, install it first:

#### Linux and macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### Windows
1. Download and run [rustup-init.exe](https://rustup.rs/)
2. Follow the installation prompts
3. Restart your terminal

#### Verify Rust Installation
```bash
rustc --version
cargo --version
```

Expected output:
```
rustc 1.70.0 (90c541806 2023-05-31)
cargo 1.70.0 (ec8a8a0ca 2023-04-25)
```

## Installation Methods

### Method 1: From Source (Recommended)

This method provides the latest features and allows customization.

#### Step 1: Clone Repository
```bash
git clone https://github.com/shashank-sangule/todo_cli.git
cd todo_cli
```

#### Step 2: Build and Install
```bash
# Build optimized release version
cargo build --release

# Install to ~/.cargo/bin/
cargo install --path .
```

#### Step 3: Verify Installation
```bash
todo --version
```

#### Optional: Add to PATH
The binary is installed to `~/.cargo/bin/todo`. Ensure this directory is in your PATH:

**Linux/macOS (bash/zsh):**
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Windows (PowerShell):**
```powershell
$env:PATH += ";$env:USERPROFILE\\.cargo\\bin"
```

### Method 2: From Crates.io (Future)

Once published to crates.io, you can install directly:

```bash
cargo install todo_app
```

### Method 3: Pre-built Binaries (Future)

Download pre-built binaries from the [releases page](https://github.com/shashank-sangule/todo_cli/releases).

## Platform-Specific Instructions

### Linux

#### Ubuntu/Debian
```bash
# Install prerequisites
sudo apt update
sudo apt install curl git build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and install todo CLI
git clone https://github.com/shashank-sangule/todo_cli.git
cd todo_cli
cargo install --path .
```

#### CentOS/RHEL/Fedora
```bash
# Install prerequisites
sudo dnf install curl git gcc

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and install todo CLI
git clone https://github.com/shashank-sangule/todo_cli.git
cd todo_cli
cargo install --path .
```

### macOS

#### Manual Installation
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and install
git clone https://github.com/shashank-sangule/todo_cli.git
cd todo_cli
cargo install --path .
```

### Windows

#### Manual Installation
```powershell
# Install Rust from https://rustup.rs/
# Clone and install
git clone https://github.com/shashank-sangule/todo_cli.git
cd todo_cli
cargo install --path .
```

## Verification

### Basic Verification
```bash
# Check version
todo --version

# Check help
todo --help

# Create a test todo
todo add "Installation test"

# List todos
todo list

# Clean up
todo delete 1
```

## Troubleshooting

### Common Issues

#### Issue: "command not found: todo"

**Cause**: The binary is not in your PATH.

**Solution**:
```bash
# Check if binary exists
ls ~/.cargo/bin/todo

# Add to PATH (Linux/macOS)
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Issue: "error: failed to compile"

**Cause**: Missing build dependencies or outdated Rust version.

**Solutions**:
```bash
# Update Rust
rustup update

# Install build essentials (Linux)
sudo apt install build-essential  # Ubuntu/Debian
sudo dnf install gcc              # CentOS/RHEL/Fedora
```

## Uninstallation

### Remove Binary
```bash
# If installed with cargo install
cargo uninstall todo_app

# Or remove manually
rm ~/.cargo/bin/todo
```

### Remove Data Files
```bash
# Remove default todo file
rm todo_list.json
```

---

**For additional help, see [USAGE.md](USAGE.md) or create an issue on [GitHub](https://github.com/shashank-sangule/todo_cli/issues).**
