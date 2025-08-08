# jl-cat

A command-line tool for displaying JSON line-delimited log files in a human-friendly format with color highlighting and intelligent formatting.

## Overview

`jl-cat` processes JSON log entries and displays them with:
- **Color-coded log levels** with automatic terminal detection
- **Compact formatting** with shortened level names (info → inf, warn → wrn, etc.)
- **Clean extras display** showing additional fields with proper formatting
- **Session markers** for organizing log output
- **Error handling** for malformed JSON with helpful error messages

## Installation

### From Source (Cloned Repository)

1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd jl-proc
   ```

2. Install the `jl-cat` binary using cargo:
   ```bash
   cargo install --path jl-cat
   ```

   This will build and install the `jl-cat` executable to your cargo bin directory (usually `~/.cargo/bin/`), making it available system-wide.

3. Verify installation:
   ```bash
   jl-cat --version
   ```

### Development Build

For development or testing purposes, you can run directly from the workspace:

```bash
# Build the entire workspace
cargo build

# Run jl-cat directly
cargo run -p jl-cat -- [OPTIONS] <FILE>
```

## Usage

```bash
# Display a JSON log file
jl-cat logs.jsonl

# Process from stdin
cat logs.jsonl | jl-cat -

# Skip empty lines
jl-cat --skip-empty-lines logs.jsonl

# Add session start markers
jl-cat --session-start "Session started" logs.jsonl

# Hide extra fields beyond timestamp/level/message  
jl-cat --no-extras logs.jsonl

# Combine options
jl-cat --skip-empty-lines --session-start "New session" --no-extras logs.jsonl
```

## Command-Line Options

- `<FILE>` - Input file to process (use `-` for stdin)
- `--skip-empty-lines` - Skip empty lines in the input
- `--session-start <STRING>` - Start a new session when message begins with this string
- `--no-extras` - Skip printing additional fields beyond timestamp, level, and message
- `--help` - Show help information
- `--version` - Show version information

## Input Format

`jl-cat` expects JSON line-delimited input where each line contains a valid JSON object. Common log formats are supported:

```json
{"timestamp": "2024-01-01T10:00:00.000Z", "level": "info", "message": "Application started"}
{"timestamp": "2024-01-01T10:00:01.000Z", "level": "warn", "message": "High memory usage", "memory_usage": 85.2}
{"timestamp": "2024-01-01T10:00:02.000Z", "level": "error", "message": "Connection failed", "error": "timeout"}
```

## Output Format

```
10:00:00.000 [inf] Application started
10:00:01.000 [wrn] High memory usage
    memory_usage: 85.2
10:00:02.000 [err] Connection failed
    error: "timeout"
```

## Log Levels

Supported log levels with color coding:
- `trace` → `trc` (dim gray)
- `debug` → `dbg` (blue)  
- `info` → `inf` (green)
- `warn` → `wrn` (yellow)
- `error` → `err` (red)
- `fatal` → `ftl` (bright red)

## Dependencies

- **jl-proc** - Core JSON log processing library
- **clap** - Command-line argument parsing
- **anyhow** - Error handling

## Development

This crate is part of the `jl-proc` workspace. See the [workspace README](../README.md) for development instructions.

```bash
# Run tests for this crate specifically
cargo test -p jl-cat

# Build this crate specifically  
cargo build -p jl-cat
```

## License

MIT License - See [LICENSE](../LICENSE) for details.