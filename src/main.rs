use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

use clap::{App, Arg};
use regex::Regex;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        //line is a String, but re.find() takes an &str as an argument
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

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
                .required(false),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap(); //extracts the pattern argument
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
