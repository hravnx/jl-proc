use std::io::Write;

use crate::LogEntry;

// --------------------------------------------------------------------------

pub struct LogEntryFormatter<W: Write> {
    use_color: bool,
    writer: W,
}

impl<W: Write> LogEntryFormatter<W> {
    pub fn new(use_color: bool, writer: W) -> Self {
        LogEntryFormatter { use_color, writer }
    }

    pub fn format(&mut self, entry: &LogEntry) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{} [{:?}] {}",
            entry.timestamp_short(),
            entry.level,
            entry.message
        )?;
        Ok(())
    }

    pub fn empty_lines(&mut self, n: usize, source: &str) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{}: {} empty lines skipped -----------",
            source, n
        )?;
        Ok(())
    }

    pub fn read_error(
        &mut self,
        line_no: usize,
        source: &str,
        error: std::io::Error,
    ) -> std::io::Result<()> {
        writeln!(self.writer, "{}({}): Read error {}", source, line_no, error)?;
        Ok(())
    }

    pub fn parse_error(
        &mut self,
        line_no: usize,
        source: &str,
        error: serde_json::Error,
    ) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{}({}): Parse error {}",
            source, line_no, error
        )?;
        Ok(())
    }
}
