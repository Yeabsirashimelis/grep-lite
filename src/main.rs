use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::{App, Arg};
use regex::Regex;

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("file to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap(); //extracts the pattern argument
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
