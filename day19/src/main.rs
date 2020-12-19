#![feature(str_split_once)]

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::rc::Rc;
use regex::Regex;

enum Rule {
    Single(String),
    Dual(Rc<Rule>, Rc<Rule>),
    Choice(Rc<Rule>, Rc<Rule>),
    Simple(String),
}

impl Rule {
    fn parse(spec: &str) -> Rule {
        if let Some((first, second)) = spec.split_once(" | ") {
            Rule::Choice(
                Rc::new(Rule::parse(first)),
                Rc::new(Rule::parse(second))
            )
        } else if let Some((first, second)) = spec.split_once(" ") {
            Rule::Dual(
                Rc::new(Rule::parse(first)),
                Rc::new(Rule::parse(second))
            )
        } else if let Ok(_) = spec.parse::<u32>() {
            Rule::Single(spec.to_string())
        } else {
            Rule::Simple(spec.to_string())
        }
    }

    fn to_pattern(&self, rule_map: &HashMap<String, Rule>) -> String {
        match self {
            Rule::Single(id) => rule_map.get(id).unwrap().to_pattern(rule_map),
            Rule::Dual(a, b) => {
                format!("(?:{}{})", a.to_pattern(rule_map), b.to_pattern(rule_map))
            }
            Rule::Choice(a, b) => {
                format!("(?:(?:{})|(?:{}))", a.to_pattern(rule_map), b.to_pattern(rule_map))
            }
            Rule::Simple(c) => (&c[1..=1]).to_string(),
        }
    }
}

fn main() {
    let rules_reader = BufReader::new(File::open("rules.txt")
        .expect("Cannot open input"));

    let mut rule_map = HashMap::new();
    for rule_line in rules_reader.lines() {
        let rule_line = rule_line.unwrap();

        let (id, rule_spec) = rule_line.split_once(": ").unwrap();
        let rule = Rule::parse(rule_spec);
        rule_map.insert(id.to_string(), rule);
    }

    let rule_zero = rule_map.get("0").unwrap();
    let pattern = format!("\\A{}\\z", rule_zero.to_pattern(&rule_map));
    let regex = Regex::new(&pattern).unwrap();

    let messages_reader = BufReader::new(File::open("messages.txt")
        .expect("Cannot open input"));
    let num_valid = messages_reader.lines()
        .map(|line| line.unwrap())
        .filter(|line| regex.is_match(line))
        .count();
    println!("Part 1: {}", num_valid);

    let rule_42 = rule_map.get("42").unwrap();
    let pattern_42 = format!("\\A{}", rule_42.to_pattern(&rule_map));
    let re_42 = Regex::new(&pattern_42).unwrap();
    let rule_31 = rule_map.get("31").unwrap();
    let pattern_31 = format!("\\A{}", rule_31.to_pattern(&rule_map));
    let re_31 = Regex::new(&pattern_31).unwrap();

    let messages_reader = BufReader::new(File::open("messages.txt")
        .expect("Cannot open input"));
    let num_valid2 = messages_reader.lines()
        .map(|line| line.unwrap())
        .filter(|line| line.len() % 8 == 0)
        .filter(|line| {
            let mut end = 0;
            let mut first_count = 0;
            while let Some(caps) = re_42.captures(&line[end..]) {
                end += (&caps[0]).len();
                first_count += 1;
            }
            let mut second_count = 0;
            while let Some(caps) = re_31.captures(&line[end..]) {
                end += (&caps[0]).len();
                second_count += 1;
            }
            end == line.len() && first_count > second_count && second_count > 0
        })
        .count();
    println!("Part 2: {}", num_valid2);
}
