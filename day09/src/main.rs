
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let numbers: Vec<u64> = reader.lines().map(|l| {
        let line = l.unwrap();
        line.parse().expect("Could not parse line")
    }).collect();

    let mut active_numbers = HashSet::new();
    for preamble_num in numbers[0..25].iter() {
        active_numbers.insert(preamble_num);
    }

    let mut target:u64 = 0;
    for (i, num) in numbers[25..numbers.len()].iter().enumerate() {
        let mut found = false;
        for candidate in active_numbers.iter() {
            if candidate > &num {
                continue;
            }
            let remainder = num - *candidate;
            if candidate != &&remainder && active_numbers.contains(&remainder) {
                found = true;
                break;
            }
        }
        if !found {
            println!("Could not find sum for {} at {}", num, i + 25);
            target = *num;
            break;
        }
        let outgoing = &numbers[i];
        active_numbers.remove(outgoing);
        active_numbers.insert(num);
    }

    for i in 0..numbers.len() {
        let mut total = 0u64;
        let mut j = i;
        let mut vec = vec![];
        while total < target {
            vec.push(numbers[j]);
            total += numbers[j];
            j += 1;
        }
        if total == target {
            vec.sort();
            println!("Sum of highest and lowest numbers in continguous run totaling {} = {}", target, vec.first().unwrap() + vec.last().unwrap());
            break;
        }
    }
}
