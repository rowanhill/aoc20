use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("src/input")
        .expect("Cannot open input"));

    let nums: Vec<i32> = reader.lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    
    for a in 0..(nums.len() - 2) {
        for b in a..(nums.len() - 1) {
            if nums[a] + nums[b] == 2020 {
                println!("Part one: {} x {} = {}", nums[a], nums[b], nums[a]*nums[b]);
            }
            for c in b..(nums.len()) {
                if nums[a] + nums[b] + nums[c] == 2020 {
                    println!("Part two: {} x {} x {} = {}", nums[a], nums[b], nums[c], nums[a]*nums[b]*nums[c]);
                }
            }
        }
    }
}
