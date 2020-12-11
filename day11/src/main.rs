use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
enum Tile {
    Floor,
    Empty,
    Occupied
}

struct Plan {
    tiles: Vec<Vec<Tile>>,
}

impl Plan {
    fn new(path: &str) -> Plan {
        let reader = BufReader::new(File::open(path)
            .expect("Cannot open input"));
        let tiles = reader.lines().map(|line| {
            let line = line.unwrap();
            line.chars().map(|c| {
                match c {
                    '.' => Tile::Floor,
                    'L' => Tile::Empty,
                    '#' => Tile::Occupied,
                    _ => panic!("Unexpected tile")
                }
            }).collect()
        }).collect();
        Plan { tiles }
    }

    fn stabilise(&mut self, part2: bool) {
        while self.step(part2) {
            // no-op
        }
    }

    fn step(&mut self, part2: bool) -> bool {
        let occ_limit = if part2 { 5 } else { 4 };
        let mut changed = false;
        self.tiles = self.tiles.iter().enumerate().map(|(y,row)| {
            row.iter().enumerate().map(|(x,tile)| {
                if let Tile::Floor = tile {
                    Tile::Floor
                } else {
                    let occ_neighbs =
                        if !part2 {
                            self.count_occupied_neighbours(x, y)
                        } else {
                            self.count_occupied_visible_neighbours(x, y)
                        };
                    if let Tile::Empty = tile {
                        if occ_neighbs == 0 {
                            changed = true;
                            Tile::Occupied
                        } else {
                            Tile::Empty
                        }
                    } else { // occupied
                        if occ_neighbs >= occ_limit {
                            changed = true;
                            Tile::Empty
                        } else {
                            Tile::Occupied
                        }
                    }
                }
            }).collect()
        }).collect();
        changed
    }

    fn count_occupied_neighbours(&self, x: usize, y: usize) -> usize {
        ((y as i32 - 1)..(y as i32 + 2)).map(|ny| {
            if ny < 0 || ny >= self.tiles.len() as i32 {
                0
            } else {
                let row = &self.tiles[ny as usize];
                ((x as i32 - 1)..(x as i32 + 2)).map(|nx| {
                    if nx < 0 || nx >= row.len() as i32 {
                        0
                    } else {
                        if nx == x as i32 && ny == y as i32 {
                            0
                        } else {
                            if let Tile::Occupied = row[nx as usize] {
                                1
                            } else {
                                0
                            }
                        }
                    }
                }).sum()
            }
        }).sum()
    }

    fn count_occupied_visible_neighbours(&self, x: usize, y: usize) -> usize {
        let dirs = [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];
        let mut count = 0;
        for (dx, dy) in &dirs {
            // println!(" Checking {},{}", dx, dy);
            let mut tile = &Tile::Floor;
            let mut nx = x as i32 + dx;
            let mut ny = y as i32 + dy;
            while let Tile::Floor = tile {
                if ny >= 0 && nx >= 0 && ny < self.tiles.len() as i32 && nx < self.tiles[0].len() as i32 {
                    tile = &self.tiles[ny as usize][nx as usize];
                } else {
                    break;
                }
                nx += dx;
                ny += dy;
            }
            if let Tile::Occupied = tile {
                count += 1;
            }
        }
        count
    }

    fn count_occupied(&self) -> usize {
        self.tiles.iter().map(|row| {
            row.iter().map(|tile| {
                match tile {
                    Tile::Occupied => 1,
                    _ => 0
                }
            }).sum::<usize>()
        }).sum()
    }
}

fn main() {
    let mut plan1 = Plan::new("input");
    plan1.stabilise(false);
    println!("Part 1, total occupied: {}", plan1.count_occupied());

    let mut plan2 = Plan::new("input");
    plan2.stabilise(true);
    println!("Part 2, total occupied: {}", plan2.count_occupied());
}