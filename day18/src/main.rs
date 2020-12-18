use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Copy, Clone)]
enum Operation {
    Plus,
    Times
}

fn evaluate(chars: &mut Peekable<Chars>) -> u64 {
    let mut op = None;
    let mut val: Option<u64> = None;
    while let Some(c) = chars.next() {
        match c {
            ' ' => continue,
            '+' => op = Some(Operation::Plus),
            '*' => op = Some(Operation::Times),
            '(' => {
                let next_term = evaluate(chars);
                if val.is_some() {
                    let prev_val = val.expect("Value must be initialised");
                    let op = &op.expect("Operation must be initialised");
                    val = match op {
                        Operation::Plus => Some(prev_val + next_term),
                        Operation::Times => Some(prev_val * next_term)
                    };
                } else {
                    val = Some(next_term);
                }
            }
            ')' => return val.expect("Value must be initialised to return it"),
            _ => {
                let mut num_str = String::new();
                num_str.push(c);
                while let Some(next_c) = chars.peek() {
                    match next_c {
                        '0'..='9' => {
                            let c = chars.next().unwrap();
                            num_str.push(c);
                        }
                        _ => {
                            break;
                        }
                    }
                }
                let num: u64 = num_str.parse().expect("Could not parse number");
                if val.is_none() {
                    val = Some(num);
                } else {
                    let op = &op.expect("Operation must be initialised if parsing second number");
                    val = match op {
                        Operation::Plus => Some(val.unwrap() + num),
                        Operation::Times => Some(val.unwrap() * num),
                    };
                }
            }
        }
    }
    return val.unwrap();
}

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let mut answers = vec![];
    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let num = evaluate(&mut line.chars().peekable());
        println!("{}", num);
        answers.push(num);
    }

    println!("Part 1: {}", answers.iter().sum::<u64>());
}
