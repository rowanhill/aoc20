use std::collections::{HashSet, HashMap};
use std::io::{BufReader, BufRead};
use std::fs::File;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"(?:^(?P<parent>.+?) bags contain)|(?:(?P<num>\d+) (?P<colour>.+?) bags?)").unwrap();
}

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let mut children_by_colour = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut captures = RULE_RE.captures_iter(&line);

        let parent_captures = captures.next().expect("Not capture groups for parent");
        let parent_colour = parent_captures.name("parent").expect("No match for parent").as_str();
        // println!("Parent: {}", parent_colour);

        let children = captures.map(|child_captures| {
            let num_str = child_captures.name("num").expect("No match for num").as_str();
            let num: i32 = num_str.parse().expect("Could not parse num as i32");
            let colour = child_captures.name("colour").expect("No match for colour").as_str();
            // println!("  Child: {} x {}", num, colour);
            (num, colour.to_string())
        }).collect::<Vec<_>>();
        children_by_colour.insert(parent_colour.to_string(), children);
    }

    let mut parents_by_colour = HashMap::new();
    for (colour, _) in &children_by_colour {
        parents_by_colour.insert(colour.clone(), vec![]);
    }
    for (colour, children) in &children_by_colour {
        for (_, child) in children {
            parents_by_colour.get_mut(child).unwrap().push(colour.clone());
        }
    }

    let mut visited = HashSet::new();
    traverse_parents("shiny gold", &parents_by_colour, &mut visited);
    println!("Part 1: {}", visited.len() - 1);

    let children_count = count_children("shiny gold", &children_by_colour);
    println!("Part 2: {}", children_count);
}

fn traverse_parents(
    colour: &str,
    parents_by_colour: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) {
    visited.insert(colour.to_string());

    let children = parents_by_colour.get(colour).unwrap();
    for child in children {
        if !visited.contains(child) {
            traverse_parents(child, parents_by_colour, visited);
        }
    }
}

fn count_children(
    colour: &str,
    children_by_colour: &HashMap<String, Vec<(i32, String)>>,
) -> i32 {
    let mut count = 0;
    for (num, child_colour) in children_by_colour.get(colour).unwrap() {
        count += num + (num * count_children(child_colour, children_by_colour));
    }
    count
}
