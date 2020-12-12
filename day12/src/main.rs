use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    // North, East, South, West
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut pos = (0, 0);
    let mut dir_index = 1; // Start East

    let mut ship_pos = (0, 0);
    let mut way_pos = (10, -1);

    for line in reader.lines() {
        let line = line.expect("Could not unwrap line");
        let command = &line[0..1];
        let value = &line[1..line.len()].parse::<i32>().expect("Could not parse line");

        let delta: &(i32, i32) = match command {
            "N" => &dirs[0],
            "E" => &dirs[1],
            "S" => &dirs[2],
            "W" => &dirs[3],
            "F" => &dirs[dir_index],
            "R" => { dir_index = (dir_index + (value / 90) as usize) % 4; &(0, 0) },
            "L" => { dir_index = (dir_index + 4 - (value / 90) as usize) % 4; &(0, 0) },
            _ => panic!("Unexpected command {}", command)
        };
        pos = (pos.0 + delta.0 * value, pos.1 + delta.1 * value);

        match command {
            "N" => { way_pos = (way_pos.0, way_pos.1 - value); },
            "E" => { way_pos = (way_pos.0 + value, way_pos.1); },
            "S" => { way_pos = (way_pos.0, way_pos.1 + value); },
            "W" => { way_pos = (way_pos.0 - value, way_pos.1); },
            "F" => { ship_pos = (ship_pos.0 + way_pos.0*value, ship_pos.1 + way_pos.1*value); }
            "R" => { for _ in 0..(value/90) { way_pos = (-way_pos.1, way_pos.0); } },
            "L" => { for _ in 0..(value/90) { way_pos = (way_pos.1, -way_pos.0); } },
            _ => panic!("Unexpected command {}", command)
        };
    }
    println!("Part 1 {}", pos.0.abs() + pos.1.abs());
    println!("Part 2 {}", ship_pos.0.abs() + ship_pos.1.abs());
}
