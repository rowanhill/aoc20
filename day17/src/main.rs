use std::collections::{HashSet};
use std::ops::RangeInclusive;

type Vect3 = (i32, i32, i32);
type Bounds3 = (RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>);
type Vect4 = (i32, i32, i32, i32);
type Bounds4 = (RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>, RangeInclusive<i32>);

fn main() {
    let input = "##..#.#.
#####.##
#######.
#..#..#.
#.#...##
..#....#
....#..#
..##.#..";

    let mut active_cubes = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((x as i32, y as i32, 0));
            }
        }
    }
    for _ in 0..6 {
        active_cubes = step(&active_cubes, get_bounds(&active_cubes));
    }
    println!("Part 1: {}", active_cubes.len());

    let mut active_cubes4 = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes4.insert((x as i32, y as i32, 0, 0));
            }
        }
    }
    for _ in 0..6 {
        active_cubes4 = step4(&active_cubes4, get_bounds4(&active_cubes4));
    }
    println!("Part 2: {}", active_cubes4.len());
}

fn get_bounds(active_cubes: &HashSet<Vect3>) -> Bounds3 {
    let mut min_x = 100000;
    let mut min_y = 100000;
    let mut min_z = 100000;
    let mut max_x = -100000;
    let mut max_y = -100000;
    let mut max_z = -100000;

    for (x, y, z) in active_cubes {
        if min_x > x - 1 {
            min_x = x - 1;
        }
        if min_y > y - 1 {
            min_y = y - 1;
        }
        if min_z > z - 1 {
            min_z = z - 1;
        }
        if max_x < x + 1 {
            max_x = x + 1;
        }
        if max_y < y + 1 {
            max_y = y + 1;
        }
        if max_z < z + 1 {
            max_z = z + 1;
        }
    }

    (min_x..=max_x, min_y..=max_y, min_z..=max_z)
}

fn count_neighbours(active_cubes: &HashSet<Vect3>, cube: Vect3) -> usize {
    let mut active_neighbours = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx != 0 || dy != 0 || dz != 0 {
                    if active_cubes.contains(&(cube.0 + dx, cube.1 + dy, cube.2 + dz)) {
                        active_neighbours += 1;
                    }
                }
            }
        }
    }
    active_neighbours
}

fn step(active_cubes: &HashSet<Vect3>, bounds: Bounds3) -> HashSet<Vect3> {
    let mut result = HashSet::new();

    let (xs, ys, zs) = bounds;

    for x in xs.clone() {
        for y in ys.clone() {
            for z in zs.clone() {
                let n = count_neighbours(active_cubes, (x, y, z));
                let curr_active = active_cubes.contains(&(x, y, z));
                let next_active = (curr_active && (n == 2 || n == 3)) || (!curr_active && n == 3);
                if next_active {
                    result.insert((x, y, z));
                }
            }
        }
    }

    result
}



fn get_bounds4(active_cubes: &HashSet<Vect4>) -> Bounds4 {
    let mut min_x = 100000;
    let mut min_y = 100000;
    let mut min_z = 100000;
    let mut min_w = 100000;
    let mut max_x = -100000;
    let mut max_y = -100000;
    let mut max_z = -100000;
    let mut max_w = -100000;

    for (x, y, z, w) in active_cubes {
        if min_x > x - 1 {
            min_x = x - 1;
        }
        if min_y > y - 1 {
            min_y = y - 1;
        }
        if min_z > z - 1 {
            min_z = z - 1;
        }
        if min_w > w - 1 {
            min_w = w - 1;
        }
        if max_x < x + 1 {
            max_x = x + 1;
        }
        if max_y < y + 1 {
            max_y = y + 1;
        }
        if max_z < z + 1 {
            max_z = z + 1;
        }
        if max_w < w + 1 {
            max_w = w + 1;
        }
    }

    (min_x..=max_x, min_y..=max_y, min_z..=max_z, min_w..=max_w)
}

fn count_neighbours4(active_cubes: &HashSet<Vect4>, cube: Vect4) -> usize {
    let mut active_neighbours = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                        if active_cubes.contains(&(cube.0 + dx, cube.1 + dy, cube.2 + dz, cube.3 + dw)) {
                            active_neighbours += 1;
                        }
                    }
                }
            }
        }
    }
    active_neighbours
}

fn step4(active_cubes: &HashSet<Vect4>, bounds: Bounds4) -> HashSet<Vect4> {
    let mut result = HashSet::new();

    let (xs, ys, zs, ws) = bounds;

    for x in xs.clone() {
        for y in ys.clone() {
            for z in zs.clone() {
                for w in ws.clone() {
                    let n = count_neighbours4(active_cubes, (x, y, z, w));
                    let curr_active = active_cubes.contains(&(x, y, z, w));
                    let next_active = (curr_active && (n == 2 || n == 3)) || (!curr_active && n == 3);
                    if next_active {
                        result.insert((x, y, z, w));
                    }
                }
            }
        }
    }

    result
}