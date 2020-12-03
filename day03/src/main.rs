use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));
    let trees_map: Vec<Vec<bool>> = reader.lines().map(|l| {
        let line = l.expect("Could not get line");
        line.chars().map(|c| c == '#').collect()
    }).collect();

    let one_by_one = count_trees_on_slope(1, 1, &trees_map);
    let three_by_one = count_trees_on_slope(3, 1, &trees_map);
    let five_by_one = count_trees_on_slope(5, 1, &trees_map);
    let seven_by_one = count_trees_on_slope(7, 1, &trees_map);
    let one_by_two = count_trees_on_slope(1, 2, &trees_map);

    println!("Part 1: {}", three_by_one);
    println!("Part 2: {}", one_by_one * three_by_one * five_by_one * seven_by_one * one_by_two);
}

fn count_trees_on_slope(dx: usize, dy: usize, trees_map: &Vec<Vec<bool>>) -> i64 {
    let mut x = 0usize;
    let mut y = 0usize;
    let mut tree_count = 0;

    while y < trees_map.len() {
        if trees_map[y][x] {
            tree_count += 1;
        }

        x = (x + dx) % trees_map[0].len();
        y += dy;
    }

    tree_count
}
