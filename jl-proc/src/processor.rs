use std::io::Write;

use crate::{LineItem, LogEntry};

// --------------------------------------------------------------------------

/// The options for processing log entries.
pub struct ProcessorOptions {
    /// if true, skip empty lines in the input
    pub skip_empty_lines: bool,
}

// --------------------------------------------------------------------------

/// A processor for log entries that formats them as lines of text.
pub struct LogEntryProcessor {
    pub options: ProcessorOptions,
}

impl LogEntryProcessor {
    pub fn new(options: ProcessorOptions) -> Self {
        Self { options }
    }

    pub fn process_entries<W: Write>(
        &self,
        w: &mut W,
        entries: impl Iterator<Item = LineItem>,
        source: &str,
    ) -> std::io::Result<()> {
        let mut continuous_empty_lines = 0;

        for entry in entries {
            match entry {
                LineItem::Entry(log_entry) => {
                    if continuous_empty_lines > 1 {
                        writeln!(
                            w,
                            "{}: {} empty lines skipped -----------",
                            source, continuous_empty_lines
                        )?;
                        continuous_empty_lines = 0;
                    }
                    self.process_entry(w, log_entry)?;
                }
                LineItem::EmptyLine(_) => {
                    continuous_empty_lines += 1;
                }
                LineItem::ReadError(line_no, e) => {
                    writeln!(w, "{}({}): Read error {}", source, line_no, e)?;
                }
                LineItem::ParseError(line_no, e) => {
                    writeln!(w, "{}({}): Parse error {}", source, line_no, e)?;
                }
            }
        }
        Ok(())
    }

    fn process_entry<W: Write>(&self, w: &mut W, entry: LogEntry) -> std::io::Result<()> {
        writeln!(
            w,
            "{} [{:?}] {}",
            entry.timestamp_short(),
            entry.level,
            entry.message
        )?;
        Ok(())
    }
}

// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::SeverityLevel as SL;

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
        let result = processor.process_entries(&mut output, entries.into_iter(), "test.log");
        assert!(result.is_ok());
        let output_str = String::from_utf8(output).unwrap();
        let expected = "10:32:51.123 [Info] A log message\n\
10:32:53.456 [Warn] Another log message\n";
        assert_eq!(output_str, expected);
    }
}
