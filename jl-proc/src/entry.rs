use std::collections::HashMap;
use std::io::{BufRead, Lines};

use serde::Deserialize;

// --------------------------------------------------------------------------

/// Represents the severity level of a log entry.
///
/// It is roughly taken from the npm logging levels, except for 'http' and
/// 'timing', which aren't really levels but categories.
///
/// See https://docs.npmjs.com/cli/v8/using-npm/logging
#[derive(Deserialize, Debug, PartialEq)]
pub enum SeverityLevel {
    #[serde(rename = "fatal")]
    Fatal,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "verbose")]
    Verbose,
    #[serde(rename = "silly")]
    Silly,
}

/// A single log entry from a file/stream of json line-delimited log entries.
///
/// ### Examples
/// ```
/// use jl_proc::*;
///
/// let json = r#"{
///     "timestamp": "2024-03-15T12:34:56.123Z",
///     "level": "info",
///     "message": "This is a log message",
///     "user_id": 42,
///     "session_id": "abc123"
/// }"#;
///
/// let log_entry: LogEntry = serde_json::from_str(json).unwrap();
/// assert_eq!(log_entry.timestamp, "2024-03-15T12:34:56.123Z");
/// assert_eq!(log_entry.timestamp_short(), "12:34:56.123");
/// assert_eq!(log_entry.level, SeverityLevel::Info);
/// assert_eq!(log_entry.message, "This is a log message");
/// assert_eq!(log_entry.extra.len(), 2);
/// ```
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: SeverityLevel,
    pub message: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl LogEntry {
    /// Returns the timestamp as a slice in 'shortened' ISO 8601 format.
    ///
    /// ### Examples
    /// ```
    /// use jl_proc::*;
    /// let json = r#"{
    ///     "timestamp": "2024-03-15T12:34:56.123Z",
    ///     "level": "info",
    ///     "message": "This is a log message"
    /// }"#;
    /// let log_entry: LogEntry = serde_json::from_str(json).unwrap();
    /// assert_eq!(log_entry.timestamp_short(), "12:34:56.123");
    /// ```
    pub fn timestamp_short(&self) -> &str {
        &self.timestamp[11..23]
    }
}

// --------------------------------------------------------------------------

pub enum LineItem {
    Entry(LogEntry),
    EmptyLine(usize),
    ReadError(usize, std::io::Error),
    ParseError(usize, serde_json::Error),
}

/// An iterator over log entries from a buffered reader.
///
/// ### Examples
/// ```no_run
/// use std::io::{BufReader, BufRead};
/// use jl_proc::{LogEntryIterator, LineItem, LogEntry};
/// let input = BufReader::new("...".as_bytes());
/// let entries: Vec<_> = LogEntryIterator::from_buf_reader(input).collect();
/// ```
pub struct LogEntryIterator<B: BufRead> {
    lines: Lines<B>,
    line_no: usize,
    is_error: bool,
}

impl<B: BufRead> LogEntryIterator<B> {
    pub fn from_buf_reader(reader: B) -> Self {
        Self {
            lines: reader.lines(),
            line_no: 0,
            is_error: false,
        }
    }
}

impl<B> Iterator for LogEntryIterator<B>
where
    B: BufRead,
{
    type Item = LineItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_error {
            // If we previously encountered an error, we skip further
            // processing.
            return None;
        }
        use LineItem as L;
        match self.lines.next() {
            Some(Ok(line)) => {
                self.line_no += 1;
                Some(if line.is_empty() {
                    L::EmptyLine(self.line_no)
                } else {
                    match serde_json::from_str::<LogEntry>(&line) {
                        Ok(entry) => L::Entry(entry),
                        Err(e) => L::ParseError(self.line_no, e),
                    }
                })
            }
            Some(Err(e)) => {
                self.line_no += 1;
                // we set the error flag to true so that we don't continue
                // processing further lines after an error. This prevents
                // infinite looping in case of a persistent read error.
                self.is_error = true;
                Some(L::ReadError(self.line_no, e))
            }
            None => None,
        }
    }
}

// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn iterator_handles_entries() {
        let raw = r#"{"timestamp": "2024-03-15T12:34:56.123Z", "level": "info", "message": "This is a log message"}

{bad}
{"timestamp": "2024-03-15T12:34:56.123Z", "level": "info", "message": "This is a log message"}

{"timestamp": "2024-03-15T12:34:56.123Z", "level": "info", "message": "This is a log message"}"#
            .to_owned();

        let entries =
            LogEntryIterator::from_buf_reader(BufReader::new(raw.as_bytes())).collect::<Vec<_>>();
        assert_eq!(entries.len(), 6);
        assert!(matches!(entries[0], LineItem::Entry(_)));
        assert!(matches!(entries[1], LineItem::EmptyLine(2)));
        assert!(matches!(entries[2], LineItem::ParseError(3, _)));
        assert!(matches!(entries[3], LineItem::Entry(_)));
        assert!(matches!(entries[4], LineItem::EmptyLine(5)));
        assert!(matches!(entries[5], LineItem::Entry(_)));
    }

    struct ErrorReader;

    impl std::io::Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("Simulated error"))
        }
    }

    #[test]
    fn iterator_handles_read_errors() {
        let input = BufReader::new(ErrorReader);
        let entries: Vec<_> = LogEntryIterator::from_buf_reader(input).collect();
        assert_eq!(entries.len(), 1);
        assert!(matches!(entries[0], LineItem::ReadError(1, _)));
    }

    #[test]
    fn can_deserialize_log_entry() {
        let json = r#"{
            "timestamp": "2024-03-15T12:34:56.042Z",
            "level": "info",
            "message": "This is a log message",
            "user_id": 42,
            "session_id": "abc123"
        }"#;

        let log_entry: LogEntry = serde_json::from_str(json).unwrap();
        assert_eq!(log_entry.timestamp, "2024-03-15T12:34:56.042Z");
        assert_eq!(log_entry.timestamp_short(), "12:34:56.042");
        assert_eq!(log_entry.level, SeverityLevel::Info);
        assert_eq!(log_entry.message, "This is a log message");
        assert_eq!(log_entry.extra.len(), 2);
        assert_eq!(
            log_entry.extra.get("user_id").unwrap(),
            &serde_json::Value::from(42)
        );
        assert_eq!(
            log_entry.extra.get("session_id").unwrap(),
            &serde_json::Value::from("abc123")
        );
    }
}
