use itertools::{Itertools, Position};
use regex::Regex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufReader, BufRead};
use phf::phf_map;

lazy_static! { static ref REGEX: Regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap(); }

const DICT: phf::Map<&'static str, &'static str> = phf_map! {
    "one"   => "1", 
    "two"   => "2",
    "three" => "3",
    "four"  => "4",
    "five"  => "5",
    "six"   => "6",
    "seven" => "7",
    "eight" => "8",
    "nine"  => "9",
};

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
                    // Check for overlapping
                    if let Some(real_last) = REGEX.find_at(&line_result, m.start() + 1) {
                        last = &line_result[real_last.range()]; 
                    }
                },
                _ => {}
            }
        }

        if first.len() > 1 {
            first = *DICT.get(first).unwrap();
        }
        if last.len() > 1 {
            last = *DICT.get(last).unwrap();
        }

        if last.is_empty() { last = first; }
        results.push((first.to_owned() + last).parse::<i32>().expect("Could not parse"));
    }

    println!("Answer: {}", results.iter().sum::<i32>());

    Ok(())
}
