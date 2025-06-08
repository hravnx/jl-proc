use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

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
