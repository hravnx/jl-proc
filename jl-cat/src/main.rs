use std::{
    fs::File,
    io::{BufReader, IsTerminal},
    path::PathBuf,
};

use jl_proc::{LogEntryFormatter, LogEntryIterator, LogEntryProcessor, ProcessorOptions};

// --------------------------------------------------------------------------

fn main() -> std::result::Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let options = ProcessorOptions {
        skip_empty_lines: cli.skip_empty_lines,
        session_start: cli.session_start.clone(),
    };

    let stdout = std::io::stdout();
    let use_color = stdout.is_terminal() && std::env::var("NO_COLOR").is_err();
    let mut formatter = LogEntryFormatter::new(use_color, stdout.lock());

    let processor = LogEntryProcessor::new(options);
    if cli.use_std_input() {
        let reader = std::io::stdin().lock();
        let entries = LogEntryIterator::from_buf_reader(reader);
        processor.process_entries(entries, "<STDIN>", &mut formatter)?;
    } else {
        let input_file = File::open(&cli.input_file)?;
        let reader = BufReader::new(input_file);
        let entries = LogEntryIterator::from_buf_reader(reader);
        processor.process_entries(
            entries,
            cli.input_file.to_str().unwrap_or("<n/a>"),
            &mut formatter,
        )?;
    };
    Ok(())
}

// --------------------------------------------------------------------------

use clap::Parser;

/// Command-line interface for showing json log entries in a human-friendly
/// format.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file to process. Use '-' for standard input.
    #[arg(value_name = "FILE")]
    input_file: PathBuf,
    /// Skip empty lines in the input.
    #[arg(long)]
    skip_empty_lines: bool,
    /// Start a new session when the message starts with this string.
    #[arg(short, long)]
    session_start: Option<String>,
}

impl Cli {
    fn use_std_input(&self) -> bool {
        self.input_file.to_str() == Some("-")
    }
}
