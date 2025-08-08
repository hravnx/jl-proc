# jl-proc

A Rust workspace for processing and formatting JSON line-delimited log entries with human-friendly output.

## Overview

This project provides tools for parsing, processing, and displaying JSON logs in a more readable format with color highlighting and intelligent formatting.

### Components

- **`jl-proc`** - Core library for JSON log processing and formatting
- **`jl-cat`** - Command-line tool for displaying JSON logs with color output ([README](jl-cat/README.md))

## Quick Start

```bash
# Build the project
cargo build

# Display a JSON log file
cargo run -p jl-cat -- logs.jsonl

# Process from stdin
echo '{"timestamp":"2024-01-01T10:00:00.000Z","level":"info","message":"Hello with extras", "host":"server-01", "port":8080}' | cargo run -p jl-cat -- -

# Skip empty lines and add session start markers
cargo run -p jl-cat -- --skip-empty-lines --session-start "Session started" logs.jsonl
```

## Features

- **Smart JSON parsing** with error handling for malformed entries
- **Color-coded output** with automatic terminal detection
- **Compact log levels** (info → inf, warn → wrn, etc.)
- **Clean extras formatting** with unquoted keys and proper indentation
- **Flexible input** from files or stdin
- **Session start markers** for log organization

## Example Output

```
09:59:59.123 [inf] Hello
10:00:00.000 [inf] Hello with extras
    host: "server-01", port: 8080
10:01:53.456 [wrn] High memory usage detected
    memory_usage: 85.2, threshold: 80.0
```

## Development

```bash
# Run tests
cargo test

# Continuous development with bacon
cargo install bacon
bacon check

# Check specific package
cargo check -p jl-proc
```

## License

MIT License - see individual crate directories for details.