# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust workspace containing tools for processing and formatting JSON line-delimited log entries. The project consists of two main components:

- **jl-proc**: A library crate that provides core functionality for parsing, processing, and formatting JSON log entries
- **jl-cat**: A command-line binary that uses jl-proc to display JSON logs in a human-friendly format with color output

## Architecture

### Core Library (jl-proc)
The library is structured around several key modules:

- `entry.rs`: Defines `LogEntry` struct and `SeverityLevel` enum for representing parsed log entries
- `iterator.rs`: Provides `LogEntryIterator` that handles reading and parsing log entries from buffered input, with error handling for malformed JSON and I/O errors
- `processor.rs`: Contains `LogEntryProcessor` that applies formatting rules and processes streams of log entries
- `formatter.rs`: Implements `LogEntryFormatter` with ANSI color support for terminal output
- `ansi.rs`: Provides macros for compile-time ANSI color code generation

### Command-Line Tool (jl-cat)
A simple CLI wrapper around jl-proc that:
- Accepts input from files or stdin
- Automatically detects terminal capabilities for color output
- Supports options like `--skip-empty-lines` and `--session-start`

## Development Commands

### Building and Testing
```bash
# Build entire workspace
cargo build

# Build specific packages
cargo build -p jl-proc
cargo build -p jl-cat

# Run tests
cargo test

# Run tests for specific package
cargo test -p jl-proc

# Check code without building
cargo check
```

### Using Bacon (File Watcher)
The project includes bacon.toml configuration for continuous development:

```bash
# Install bacon if not available
cargo install bacon

# Run continuous check
bacon check

# Run continuous tests
bacon test

# Run clippy on all targets
bacon clippy-all
```

### Running the CLI Tool
```bash
# Build and run jl-cat
cargo run -p jl-cat -- input.jsonl

# Run with stdin
echo '{"timestamp":"2024-01-01T10:00:00.000Z","level":"info","message":"test"}' | cargo run -p jl-cat -- -

# Run with options
cargo run -p jl-cat -- --skip-empty-lines --session-start "Starting" input.jsonl
```

## Code Architecture Notes

- The codebase follows a clean separation between parsing (`LogEntryIterator`), processing (`LogEntryProcessor`), and formatting (`LogEntryFormatter`)
- Error handling is built into the iterator, which yields `LineItem` enum variants for successful entries, empty lines, parse errors, and read errors
- ANSI color support is implemented through compile-time macros in `ansi.rs`
- The formatter supports both colored and plain text output based on terminal detection
- Log levels are mapped to a numeric severity system for consistent ordering and formatting

## Testing

Tests are included throughout the codebase using standard Rust `#[cfg(test)]` modules. The test suite covers:
- JSON parsing and deserialization
- Log entry iteration and error handling  
- Formatting output verification
- ANSI color macro functionality

## Commit Guidelines

- Do not mention claude or claude code in commit messages