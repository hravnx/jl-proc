use std::io::Write;

use crate::{LineItem, LogEntryFormatter};

// --------------------------------------------------------------------------

/// The options for processing log entries.
pub struct ProcessorOptions {
    /// if true, skip empty lines in the input
    pub skip_empty_lines: bool,
}

// --------------------------------------------------------------------------

/// A processor for log entries that formats them as lines of text.
pub struct LogEntryProcessor {
    /// The options for processing log entries.
    pub options: ProcessorOptions,
}

impl LogEntryProcessor {
    pub fn new(options: ProcessorOptions) -> Self {
        Self { options }
    }

    pub fn process_entries<W: Write>(
        &self,
        entries: impl Iterator<Item = LineItem>,
        source: &str,
        fmt: &mut LogEntryFormatter<W>,
    ) -> std::io::Result<()> {
        let mut continuous_empty_lines = 0;

        for entry in entries {
            match entry {
                LineItem::Entry(log_entry) => {
                    if continuous_empty_lines > 1 {
                        fmt.empty_lines(continuous_empty_lines, source)?;
                        continuous_empty_lines = 0;
                    }
                    fmt.format(&log_entry)?;
                }
                LineItem::EmptyLine(_) => {
                    continuous_empty_lines += 1;
                }
                LineItem::ReadError(line_no, e) => {
                    fmt.read_error(line_no, source, e)?;
                }
                LineItem::ParseError(line_no, e) => {
                    fmt.parse_error(line_no, source, e)?;
                }
            }
        }
        Ok(())
    }
}

// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{LogEntry, SeverityLevel as SL};

    use super::*;

    #[test]
    fn shows_sources_when_they_change() {
        let options = ProcessorOptions {
            skip_empty_lines: false,
        };
        let entries = vec![
            LineItem::Entry(LogEntry {
                timestamp: "2024-01-01T10:32:51.123Z".into(),
                level: SL::Info,
                message: "A log message".into(),
                extra: HashMap::default(),
            }),
            LineItem::Entry(LogEntry {
                timestamp: "2024-01-01T10:32:53.456Z".into(),
                level: SL::Warn,
                message: "Another log message".into(),
                extra: HashMap::default(),
            }),
        ];
        let processor = LogEntryProcessor::new(options);
        let mut output = Vec::new();
        let mut formatter = LogEntryFormatter::new(false, &mut output);
        let result = processor.process_entries(entries.into_iter(), "test.log", &mut formatter);
        assert!(result.is_ok());
        let output_str = String::from_utf8(output).unwrap();
        let expected = "10:32:51.123 [Info] A log message\n\
10:32:53.456 [Warn] Another log message\n";
        assert_eq!(output_str, expected);
    }
}
