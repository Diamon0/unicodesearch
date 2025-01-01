use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

use clap::{CommandFactory, Parser};
use regex::Regex;

// A relatively straight-forward and mostly featureless program to search for unicode codes
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Query to search for in the Unicode list
    query: String,

    #[arg(
        short = 'f',
        long,
        value_name = "FILE",
        default_value = "/usr/share/unicode/UnicodeData.txt"
    )]
    unicode_file: Option<PathBuf>,
}

fn search_file_for_string(path: PathBuf, search_str: &str) -> io::Result<Vec<[String; 2]>> {
    // Open the file
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Create a case-insensitive regex from the search string
    let regex = Regex::new(&format!("(?i){}", regex::escape(search_str))).expect("Invalid regex");

    // Collect matching lines
    let matches = reader
        .lines()
        .filter_map(Result::ok) // Ignore lines that fail to read
        .filter(|line| regex.is_match(line))
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() >= 2 {
                Some([parts[0].to_string(), parts[1].to_string()])
            } else {
                None
            }
        })
        .collect();

    Ok(matches)
}

fn main() {
    let cli = Args::parse();

    let path = cli.unicode_file.unwrap();

    if !path.exists() || !path.is_file() {
        let mut cmd = Args::command();
        cmd.error(
            clap::error::ErrorKind::Io,
            "Could not access Unicode Data file",
        )
        .exit();
    }

    match search_file_for_string(path, &cli.query) {
        Ok(matches) => {
            for entry in matches {
                println!("{}   | {} {}", char::from_u32(u32::from_str_radix(entry[0].as_str(), 16).unwrap_or(0)).unwrap_or('\u{003F}'), entry[0], entry[1]);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
