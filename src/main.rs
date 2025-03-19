use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::ExitCode;

use seqsee::formatter::FormatAnsi;
use seqsee::output::{table::TableFormatter, raw::RawFormatter};
use seqsee::parser::AnsiParser;

#[derive(Parser)]
#[command(
    name = "seqsee",
    author = "Kirill Furtikov",
    version,
    about = "A tool for parsing and displaying ANSI escape sequences in a human-readable format"
)]
struct Cli {
    /// Input file (reads from stdin if not specified)
    #[arg(short, long)]
    file: Option<PathBuf>,
    
    /// Display as a formatted table (default if no format specified)
    #[arg(long, short,default_value_t = false, group = "output_format")]
    table: bool,
    
    /// Highlight escape sequences in the original text
    #[arg(long, short, default_value_t = false, group = "output_format")]
    raw: bool,
    
    /// Disable colorized output
    #[arg(long)]
    no_color: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    
    // Get input
    let result = match get_input(&cli.file) {
        Ok(input) => {
            match process_input(input, &cli) {
                Ok(output) => {
                    println!("{}", output);
                    ExitCode::SUCCESS
                },
                Err(err) => {
                    eprintln!("Error processing input: {}", err);
                    ExitCode::FAILURE
                }
            }
        },
        Err(err) => {
            eprintln!("Error reading input: {}", err);
            ExitCode::FAILURE
        }
    };
    
    result
}

fn get_input(file_path: &Option<PathBuf>) -> io::Result<Box<dyn Read>> {
    match file_path {
        Some(path) => {
            let file = File::open(path)?;
            Ok(Box::new(file))
        },
        None => Ok(Box::new(io::stdin())),
    }
}

fn process_input(input: Box<dyn Read>, cli: &Cli) -> io::Result<String> {
    let colorize = !cli.no_color;
    
    // Parse ANSI sequences
    let elements = match AnsiParser::parse(input) {
        Ok(elems) => elems,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, format!("{}", err))),
    };
    
    // Format according to the selected mode
    let output = if cli.raw {
        let formatter = RawFormatter::new(colorize);
        formatter.format(&elements)
    } else {
        // Default to table mode
        let formatter = TableFormatter::new(colorize);
        formatter.format(&elements)
    };
    
    Ok(output)
} 