extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    lazy_static! {
        static ref PASS_LINE_RE: Regex = Regex::new(r"(\d+)-(\d+) (.+): (.+)").unwrap();
    }
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let count = reader.lines()
        .fold((0, 0), |(part1, part2), line_result| {
            let line = line_result.unwrap();
            let caps = PASS_LINE_RE.captures(&line).expect("Could not find line captures");
            let min = caps[1].parse::<usize>().expect("Could not parse min");
            let max = caps[2].parse::<usize>().expect("Could not parse max");
            let letter: &str = &caps[3];
            let password: &str = &caps[4];

            let occurences = password.matches(letter).count();
            let new_part1 = if occurences >= min && occurences <= max {
                part1 + 1
            } else {
                part1
            };

            let first_char = &password[min-1..min];
            let second_char = &password[max-1..max];
            let new_part2 = if (first_char == letter) ^ (second_char == letter) {
                part2 + 1
            } else {
                part2
            };

            (new_part1, new_part2)
        });

    println!("part 1: {}", count.0);
    println!("part 2: {}", count.1);
}
