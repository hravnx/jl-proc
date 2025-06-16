use std::io::Write;

use crate::{LogEntry, ansi_color};

// --------------------------------------------------------------------------

/// A formatter for log entries that writes them to a given writer.
pub struct LogEntryFormatter<W: Write> {
    writer: W,
    timestamp_format: &'static str,
    level_table: [&'static str; 7],
    eol: &'static str,
}

impl<W: Write> LogEntryFormatter<W> {
    /// Creates a new `LogEntryFormatter`.
    pub fn new(use_color: bool, writer: W) -> Self {
        let (timestamp_format, level_table, eol) = if use_color {
            (
                ansi_color!(fg:4),
                DEFAULT_LEVEL_TABLE_COLOR,
                concat!(ansi_color!(), "\n"),
            )
        } else {
            ("", DEFAULT_LEVEL_TABLE, "\n")
        };
        Self {
            level_table,
            writer,
            eol,
            timestamp_format,
        }
    }

    /// Formats a single log entry and writes it to the writer.
    pub fn format_entry(&mut self, entry: &LogEntry) -> std::io::Result<()> {
        write!(
            self.writer,
            "{}{}",
            self.timestamp_format,
            entry.timestamp_short()
        )?;
        write!(self.writer, "{}", self.level_table[entry.level().as_u8()])?;
        write!(self.writer, "{}", entry.message)?;
        write!(self.writer, "{}", self.eol)
    }

    /// Formats a number of empty lines and writes it to the writer.
    pub fn format_empty_lines(&mut self, n: usize, source: &str) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{}: {} empty lines skipped -----------",
            source, n
        )
    }

    /// Formats a read error and writes it to the writer.
    pub fn format_read_error(
        &mut self,
        line_no: usize,
        source: &str,
        error: std::io::Error,
    ) -> std::io::Result<()> {
        writeln!(self.writer, "{}({}): Read error {}", source, line_no, error)
    }

    /// Formats a parse error and writes it to the writer.
    pub fn format_parse_error(
        &mut self,
        line_no: usize,
        source: &str,
        error: serde_json::Error,
    ) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{}({}): Parse error {}",
            source, line_no, error
        )
    }
}

// --------------------------------------------------------------------------

const DEFAULT_LEVEL_TABLE: [&str; 7] = [
    " [fatal] ", // Fatal
    " [error] ", // Error
    " [warn]  ", // Warning
    " [info]  ", // Info
    " [debug] ", // Debug
    " [verb]  ", // Verbose
    " [other] ", // Other
];

// See color table here https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
//
// Color names are from https://colornamer.robertcooper.me/
const DEFAULT_LEVEL_TABLE_COLOR: [&str; 7] = [
    concat!(ansi_color!(fg: 11, bg: 9), " [fatal] "), // Fatal ->  Yellow Red on red bg
    concat!(ansi_color!(fg: 9), " [error] "),         // Error -> Red
    concat!(ansi_color!(fg: 11), " [warn]  "),        // Warning -> Yellow
    concat!(ansi_color!(fg: 254), " [info]  "),       // Info -> Titanium White
    concat!(ansi_color!(fg: 6), " [verb]  "),         // Verbose -> Teal
    concat!(ansi_color!(fg: 27), " [debug] "),        // Debug -> Bright Blue
    concat!(ansi_color!(fg: 5), " [other] "),         // Other -> Purple
];
