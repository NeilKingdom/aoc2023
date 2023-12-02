use itertools::{Itertools, Position};
use regex::Regex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufReader, BufRead};

lazy_static! { static ref REGEX: Regex = Regex::new(r"\d").unwrap(); }

fn main() -> std::io::Result<()> {
    let fhndl = File::open("input.txt")?;
    let buf_reader = BufReader::new(fhndl);
    let mut results: Vec<i32> = Vec::new();

    for line in buf_reader.lines() {
        let line_result = line.expect("Could not read line");

        let mut first: &str = "";
        let mut last: &str = "";

        let matches = REGEX.find_iter(&line_result).with_position();
        for (idx, m) in matches {
            match idx {
                Position::First | Position::Only => {
                    first = &line_result[m.range()];
                },
                Position::Last => {
                    last = &line_result[m.range()];
                },
                _ => {}
            }
        }

        if last.is_empty() { last = first; }
        results.push((first.to_owned() + last).parse::<i32>().expect("Could not parse"));
    }

    println!("Answer: {}", results.iter().sum::<i32>());

    Ok(())
}
