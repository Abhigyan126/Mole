# Mole - Directory Tree Printer

A command-line Rust tool to print directory trees with optional file sizes.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Build Instructions](#build-instructions)
- [Usage](#usage)
- [Examples](#examples)
- [Output Format](#output-format)

## Features

- Print directory tree structure in a visually appealing format
- Display file sizes in human-readable format (B, KB, MB, GB)
- Save output to markdown files
- Cross-platform support (Linux, macOS, Windows)
- Fast and efficient recursive directory traversal

## Prerequisites

Before building Mole, you need to have the following installed:

### Rust Toolchain

Mole is written in Rust, so you'll need the Rust programming language toolchain.

**Installation via rustup (recommended):**

```bash
# Visit https://rustup.rs for automated installation
# Or use the following command:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Verify Installation:**

```bash
rustc --version
cargo --version
```

Expected output:
```
rustc 1.x.x (or newer)
cargo 1.x.x (or newer)
```

### Dependencies

Mole uses the following Rust crates (automatically handled by Cargo):

- `clap` - Command-line argument parsing

No additional system dependencies are required.

## Installation

### Method 1: Build from Source

```bash
# Clone or navigate to the project directory
cd mole

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

### Method 2: Quick Build

```bash
# Debug build (faster, for testing)
cargo build

# Release build (optimized, production use)
cargo build --release

# Binary only build (fastest)
cargo build --bin mole
```

### Method 3: Run Without Installing

```bash
# Run directly using cargo
cargo run -- --path <DIRECTORY>

# With release optimizations
cargo run --release -- --path <DIRECTORY>
```

## Build Instructions

### Basic Build Commands

```bash
# Debug build - compiles quickly, no optimizations
cargo build

# Release build - optimized binary, slower compilation
cargo build --release

# Binary only build - faster than full build
cargo build --bin mole

# Clean and rebuild
cargo clean && cargo build
```

### Build Output

After building, the executable can be found at:

| Build Type | Path |
|------------|------|
| Debug | `./target/debug/mole` |
| Release | `./target/release/mole` |

### Cross-Compilation

To build for a different target:

```bash
# Example: build for Windows on Linux
cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

### Command Syntax

```bash
mole [OPTIONS] --path <PATH>
```

### Options

| Short | Long | Description | Default |
|-------|------|-------------|---------|
| `-p` | `--path` | Path to the directory to display | Current directory (`.`) |
| `-s` | `--size` | Show file sizes in human-readable format | Disabled |
| `-o` | `--save` | Save output to a markdown file | None (print to stdout) |
| `-h` | `--help` | Display help information | - |
| `-V` | `--version` | Display version information | - |

### Path Arguments

- `.` - Current working directory
- `..` - Parent directory
- `../sibling` - Relative path from parent
- `/absolute/path` - Absolute path
- `~/path` - Home directory (expand manually if needed)

## Examples

### Example 1: Basic Directory Tree

Print the tree structure of the current directory:

```bash
# Using long option
mole --path .

# Using short option
mole -p .

# Using current directory shortcut
mole
```

**Sample Output:**
```
├── src/
│   ├── main.rs
│   └── lib.rs
├── Cargo.toml
└── README.md
```

### Example 2: Directory Tree with Sizes

Display the tree with file sizes shown next to each file:

```bash
# Long options
mole --path /path/to/directory --size

# Short options
mole -p /path/to/directory -s
```

**Sample Output:**
```
├── src/
│   ├── main.rs | 2.50 KB
│   └── lib.rs | 1.20 KB
├── Cargo.toml | 0.50 KB
└── README.md | 4.00 KB
```

### Example 3: Save to File

Save the directory tree to a markdown file:

```bash
# Save to current directory (as mole.md)
mole --path . --save .

# Save to specific file
mole -p . -o ./directory_tree.md

# Save with sizes included
mole -p . --size -o ./tree_with_sizes.md
```

The saved file will be wrapped in markdown code blocks:

```markdown
```
├── src/
│   ├── main.rs
│   └── lib.rs
├── Cargo.toml
└── README.md
```
```

### Example 4: Full Usage

A complete example with all options:

```bash
# Print tree with sizes, save to file
mole -p /Users/username/projects/myapp -s -o ./docs/tree.md

# Or using long options
mole --path /Users/username/projects/myapp --size --save ./docs/tree.md
```

### Example 5: Using in Scripts

Use mole in shell scripts for automation:

```bash
#!/bin/bash

# Generate tree on every build
mole -p ./src -o ./docs/source_tree.md

# Check directory structure in CI/CD
mole -p . | tee build_tree.txt
```

## Output Format

### Tree Characters

Mole uses standard Unicode box-drawing characters for tree visualization:

| Character | Meaning |
|-----------|---------|
| `├──` | Branch (indicates item has siblings after it) |
| `└──` | Corner (indicates last item in directory) |
| `│` | Vertical line (continuation marker) |
| `    ` | Four spaces (padding for nested items) |

### Size Format

File sizes are displayed in human-readable format:

| Size Range | Format | Example |
|------------|--------|---------|
| < 1 KB | Bytes | `512 B` |
| 1 KB - 1 MB | Kilobytes | `25.50 KB` |
| 1 MB - 1 GB | Megabytes | `1.50 MB` |
| >= 1 GB | Gigabytes | `2.75 GB` |

### Sample Output Comparison

**Without sizes:**
```
project/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── utils/
│       └── helpers.rs
├── tests/
│   └── integration.rs
├── Cargo.toml
└── README.md
```

**With sizes:**
```
project/
├── src/
│   ├── main.rs | 3.20 KB
│   ├── lib.rs | 1.50 KB
│   └── utils/
│       └── helpers.rs | 0.80 KB
├── tests/
│   └── integration.rs | 2.10 KB
├── Cargo.toml | 1.20 KB
└── README.md | 5.00 KB
```

## Common Issues

### Issue: Permission Denied

**Problem:** Cannot read directory contents.

**Solution:** Check directory permissions or run with appropriate access rights.

```bash
# Check permissions
ls -la /path/to/directory

# Use sudo if necessary (not recommended for security)
sudo mole -p /protected/directory
```

### Issue: Path Not Found

**Problem:** Specified path does not exist.

**Solution:** Verify the path exists and is correct.

```bash
# Check if path exists
ls -la /path/to/directory

# Use absolute path
mole -p /absolute/path
```
