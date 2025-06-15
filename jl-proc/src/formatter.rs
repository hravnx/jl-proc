use std::io::Write;

use owo_colors::OwoColorize;

use crate::LogEntry;

// --------------------------------------------------------------------------

/// A formatter for log entries that writes them to a given writer.
pub struct LogEntryFormatter<W: Write> {
    use_color: bool,
    writer: W,
}

impl<W: Write> LogEntryFormatter<W> {
    /// Creates a new `LogEntryFormatter`.
    pub fn new(use_color: bool, writer: W) -> Self {
        LogEntryFormatter { use_color, writer }
    }

    /// Formats a single log entry and writes it to the writer.
    pub fn format_entry(&mut self, entry: &LogEntry) -> std::io::Result<()> {
        if self.use_color {
            ColorFormatImpl::entry(&mut self.writer, entry)
        } else {
            NoColorFormatImpl::entry(&mut self.writer, entry)
        }
    }

    /// Formats a number of empty lines and writes it to the writer.
    pub fn format_empty_lines(&mut self, n: usize, source: &str) -> std::io::Result<()> {
        if self.use_color {
            ColorFormatImpl::empty_lines(&mut self.writer, n, source)
        } else {
            NoColorFormatImpl::empty_lines(&mut self.writer, n, source)
        }
    }

    /// Formats a read error and writes it to the writer.
    pub fn format_read_error(
        &mut self,
        line_no: usize,
        source: &str,
        error: std::io::Error,
    ) -> std::io::Result<()> {
        if self.use_color {
            ColorFormatImpl::read_error(&mut self.writer, line_no, source, error)
        } else {
            NoColorFormatImpl::read_error(&mut self.writer, line_no, source, error)
        }
    }

    /// Formats a parse error and writes it to the writer.
    pub fn format_parse_error(
        &mut self,
        line_no: usize,
        source: &str,
        error: serde_json::Error,
    ) -> std::io::Result<()> {
        if self.use_color {
            ColorFormatImpl::parse_error(&mut self.writer, line_no, source, error)
        } else {
            NoColorFormatImpl::parse_error(&mut self.writer, line_no, source, error)
        }
    }
}

// --------------------------------------------------------------------------

struct ColorFormatImpl;

impl ColorFormatImpl {
    fn entry<W: Write>(mut w: W, entry: &LogEntry) -> std::io::Result<()> {
        use crate::SeverityLevel as SL;
        let level = entry.level();
        let level_str = level.as_str();
        let level = match level {
            SL::Fatal => level_str.red().into_styled(),
            SL::Error => level_str.red().into_styled(),
            SL::Warn => level_str.yellow().into_styled(),
            SL::Info => level_str.green().into_styled(),
            SL::Debug => level_str.blue().into_styled(),
            SL::Verbose => level_str.cyan().into_styled(),
            SL::Other(_) => level_str.magenta().into_styled(),
        };

        writeln!(
            w,
            "{} [{}] {}",
            entry.timestamp_short().green(),
            level,
            entry.message
        )?;
        Ok(())
    }

    fn empty_lines<W: Write>(mut w: W, n: usize, source: &str) -> std::io::Result<()> {
        writeln!(
            w,
            "{}: {} empty lines skipped -----------",
            source.bold(),
            n.to_string().yellow()
        )?;
        Ok(())
    }

    fn read_error<W: Write>(
        mut w: W,
        line_no: usize,
        source: &str,
        error: std::io::Error,
    ) -> std::io::Result<()> {
        writeln!(
            w,
            "{}({}): Read error {}",
            source.bold(),
            line_no.to_string().red(),
            error
        )?;
        Ok(())
    }

    fn parse_error<W: Write>(
        mut w: W,
        line_no: usize,
        source: &str,
        error: serde_json::Error,
    ) -> std::io::Result<()> {
        writeln!(
            w,
            "{}({}): Parse error {}",
            source.bold(),
            line_no.to_string().red(),
            error
        )?;
        Ok(())
    }
}

struct NoColorFormatImpl;

impl NoColorFormatImpl {
    fn entry<W: Write>(mut w: W, entry: &LogEntry) -> std::io::Result<()> {
        writeln!(
            w,
            "{} [{}] {}",
            entry.timestamp_short(),
            entry.level().as_str(),
            entry.message
        )?;
        Ok(())
    }

    fn empty_lines<W: Write>(mut w: W, n: usize, source: &str) -> std::io::Result<()> {
        writeln!(
            w,
            "{}: {} empty lines skipped -----------",
            source,
            n.to_string()
        )?;
        Ok(())
    }

    fn read_error<W: Write>(
        mut w: W,
        line_no: usize,
        source: &str,
        error: std::io::Error,
    ) -> std::io::Result<()> {
        writeln!(w, "{}({}): Read error {}", source, line_no, error)?;
        Ok(())
    }

    fn parse_error<W: Write>(
        mut w: W,
        line_no: usize,
        source: &str,
        error: serde_json::Error,
    ) -> std::io::Result<()> {
        writeln!(w, "{}({}): Parse error {}", source, line_no, error)?;
        Ok(())
    }
}
