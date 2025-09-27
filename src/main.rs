use std::{
    error::Error,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};
use regex::Regex;

/// Our own error type for this application.
#[derive(Debug)]
enum GrepLiteError {
    Io(io::Error),
    Regex(regex::Error),
}

impl fmt::Display for GrepLiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrepLiteError::Io(e) => write!(f, "I/O error: {}", e),
            GrepLiteError::Regex(e) => write!(f, "Regex error: {}", e),
        }
    }
}

impl Error for GrepLiteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GrepLiteError::Io(e) => Some(e),
            GrepLiteError::Regex(e) => Some(e),
        }
    }
}

// Conversions so we can use `?` with std and regex errors.
impl From<io::Error> for GrepLiteError {
    fn from(err: io::Error) -> Self {
        GrepLiteError::Io(err)
    }
}

impl From<regex::Error> for GrepLiteError {
    fn from(err: regex::Error) -> Self {
        GrepLiteError::Regex(err)
    }
}

/// Reads each line from the reader and prints lines that match the regex.
fn process_lines<T: BufRead>(reader: T, re: &Regex) -> Result<(), GrepLiteError> {
    for line_result in reader.lines() {
        let line = line_result?; // automatically becomes GrepLiteError::Io
        if re.is_match(&line) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() -> Result<(), GrepLiteError> {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("Searches for lines matching a regex pattern")
        .arg(
            Arg::with_name("pattern")
                .help("The regex pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search (defaults to stdin if omitted or \"-\")")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").expect("pattern is required");
    let re = Regex::new(pattern)?; // becomes GrepLiteError::Regex

    let input = args.value_of("input").unwrap_or("-");
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, &re)?;
    } else {
        let file = File::open(input)?; // becomes GrepLiteError::Io
        let reader = BufReader::new(file);
        process_lines(reader, &re)?;
    }

    Ok(())
}
