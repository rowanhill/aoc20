use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn parse_coord(line: String) -> (i32, i32) {
    let mut coord = (0, 0);
    let mut chars_iter = line.chars();
    while let Some(c) = chars_iter.next() {
        coord = match c {
            'e' => (coord.0 + 1, coord.1),
            's' => {
                let c2 = chars_iter.next().expect("No second letter");
                match c2 {
                    'e' => (coord.0, coord.1 + 1),
                    'w' => (coord.0 - 1, coord.1 + 1),
                    _ => panic!("Unexpected second letter: {}", c2),
                }
            },
            'w' => (coord.0 - 1, coord.1),
            'n' => {
                let c2 = chars_iter.next().expect("No second letter");
                match c2 {
                    'e' => (coord.0 + 1, coord.1 - 1),
                    'w' => (coord.0, coord.1 - 1),
                    _ => panic!("Unexpected second letter: {}", c2),
                }
            },
            _ => panic!("Unexpected first letter: {}", c),
        };
        // println!("{:?}", coord);
    }
    coord
}

const DIRS: [(i32, i32); 6] = [
    (1, 0), (1, -1), (0, -1),
    (-1, 0), (-1, 1), (0, 1),
];

fn count_black_neighbours(coord: (i32, i32), black_tiles: &HashSet<(i32, i32)>) -> usize {
    let mut count = 0;

    for (de, dse) in DIRS.iter() {
        let coord = (coord.0 + de, coord.1 + dse);
        if black_tiles.contains(&coord) {
            count += 1;
        }
    }

    count
}

fn add_neighbours(coord: (i32, i32), live_tiles: &mut HashSet<(i32, i32)>) {
    for (de, dse) in DIRS.iter() {
        let coord = (coord.0 + de, coord.1 + dse);
        live_tiles.insert(coord);
    }
}

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let mut black_tiles = HashSet::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let coord = parse_coord(line);
        if black_tiles.contains(&coord) {
            black_tiles.remove(&coord);
        } else {
            black_tiles.insert(coord.clone());
        }
    }

    println!("Part 1: {}", black_tiles.len());

    let mut live_tiles = HashSet::new();
    for coord in black_tiles.iter() {
        live_tiles.insert(coord.clone());
        add_neighbours(coord.clone(), &mut live_tiles);
    }

    for _ in 1..=100 {
        let mut new_black_tiles = HashSet::new();
        let mut new_live_tiles = HashSet::new();

        for coord in live_tiles.iter() {
            let num_black_neighbours = count_black_neighbours(coord.clone(), &black_tiles);
            if black_tiles.contains(&coord) {
                if num_black_neighbours == 1 || num_black_neighbours == 2 {
                    new_live_tiles.insert(coord.clone());
                    add_neighbours(coord.clone(), &mut new_live_tiles);
                    new_black_tiles.insert(coord.clone());
                }
                // Else: 0 or >2 neighbours, so flip to white
            } else {
                if num_black_neighbours == 2 {
                    new_live_tiles.insert(coord.clone());
                    add_neighbours(coord.clone(), &mut new_live_tiles);
                    new_black_tiles.insert(coord.clone());
                }
                // Else: not exactly 2 neighbs, so stay white
            }
        }

        black_tiles = new_black_tiles;
        live_tiles = new_live_tiles;
    }

    println!("Part 2: {}", black_tiles.len());
}
