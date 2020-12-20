use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{HashMap};

// Rotations anticlockwise
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Reflection {
    None,
    Reversed,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct TopEdge {
    data: String,
    id: u16, // The id of the tile you need to transform to give this top edge
    reflection: Reflection, // The reflection to apply to give this top edge
    rotation: Rotation, // The rotation (after reflection) to apply to give this top edge
}
impl TopEdge {
    fn new(data: String, id: u16, rotation: Rotation, reflection: Reflection) -> TopEdge {
        TopEdge { data, id, rotation, reflection }
    }
}

enum Side {
    Right, Bottom
}

#[derive(Debug)]
struct Tile {
    id: u16,
    data: Vec<String>,
}

impl Tile {
    fn new(id: u16, data: Vec<String>) -> Tile {
        Tile { id, data }
    }

    fn edge(&self, side: Side, rotation: &Rotation, reflection: &Reflection) -> String {
        let index = match side {
            Side::Right => match reflection {
                Reflection::Reversed => match rotation {
                    Rotation::Zero => 1,
                    Rotation::Ninety => 2,
                    Rotation::OneEighty => 3,
                    Rotation::TwoSeventy => 0,
                }
                Reflection::None => match rotation {
                    Rotation::Zero => 5,
                    Rotation::Ninety => 6,
                    Rotation::OneEighty => 7,
                    Rotation::TwoSeventy => 4,
                }
            }
            Side::Bottom => match reflection {
                Reflection::Reversed => match rotation {
                    Rotation::Zero => 6,
                    Rotation::Ninety => 5,
                    Rotation::OneEighty => 4,
                    Rotation::TwoSeventy => 7,
                }
                Reflection::None => match rotation {
                    Rotation::Zero => 2,
                    Rotation::Ninety => 1,
                    Rotation::OneEighty => 0,
                    Rotation::TwoSeventy => 3,
                }
            }
        };

        self.edges().get(index).unwrap().data.clone()
    }

    fn edges(&self) -> Vec<TopEdge> {
        let mut edges: Vec<TopEdge> = vec![];

        // Top line needs no modification
        let top = self.data[0].clone();

        // Bottom line would be reversed if rotated to the top
        let bottom: String = self.data[9].chars().rev().collect();

        // Right hand side must be read top to bottom to get the left-to-right string if it were
        // rotated to the top.
        let right: String = self.data.iter()
            .map(|l| l.chars().skip(9).next().unwrap())
            .collect();

        // Left hand side must be read bottom to top.
        let left: String = self.data.iter()
            .map(|l| l.chars().next().unwrap())
            .rev()
            .collect();

        // Reversed
        edges.push(TopEdge::new(top.chars().rev().collect(), self.id, Rotation::Zero, Reflection::Reversed));
        edges.push(TopEdge::new(left.chars().rev().collect(), self.id, Rotation::Ninety, Reflection::Reversed));
        edges.push(TopEdge::new(bottom.chars().rev().collect(), self.id, Rotation::OneEighty, Reflection::Reversed));
        edges.push(TopEdge::new(right.chars().rev().collect(), self.id, Rotation::TwoSeventy, Reflection::Reversed));

        // Standard
        edges.push(TopEdge::new(top, self.id, Rotation::Zero, Reflection::None));
        edges.push(TopEdge::new(right, self.id, Rotation::Ninety, Reflection::None));
        edges.push(TopEdge::new(bottom, self.id, Rotation::OneEighty, Reflection::None));
        edges.push(TopEdge::new(left, self.id, Rotation::TwoSeventy, Reflection::None));

        // if self.id == 3461 {
        //     println!("Edges for 3461: {:?}", edges);
        // }

        edges
    }

    fn print_data(&self) {
        for line in &self.data {
            println!("{}", line);
        }
    }

    fn transformed_sea_subgrid(&self, reflection: &Reflection, rotation: &Rotation) -> [[bool; 8]; 8] {
        let mut subgrid = [[false; 8]; 8];

        for y in 0..8 {
            for x in 0..8 {
                // Reflect
                let out_x = match reflection {
                    Reflection::None => x,
                    Reflection::Reversed => 7 - x,
                };
                let out_y = y;
                // Rotate
                let (out_x, out_y) = match rotation {
                    Rotation::Zero => (out_x, out_y),
                    Rotation::Ninety => (out_y, 7 - out_x), // (2, 1) -> (1, 5)
                    Rotation::OneEighty => (7 - out_x, 7 - out_y), // (2, 1) -> (5, 6)
                    Rotation::TwoSeventy => (7 - out_y, out_x), // (2, 1) -> (6, 2)
                };
                // Translate (to account for matching-data edges)
                let src_x = x+1;
                let src_y = y+1;
                // println!("{},{} -> {},{}", src_x, src_y, out_x, out_y);
                subgrid[out_y][out_x] = self.data[src_y].chars().skip(src_x).next().unwrap() == '#';
            }
        }

        subgrid
    }
}

struct EdgeMap {
    id_and_edge_by_edge_data: HashMap<String, Vec<TopEdge>>,
}
impl EdgeMap {
    fn new() -> EdgeMap {
        EdgeMap { id_and_edge_by_edge_data: HashMap::new() }
    }

    fn insert(&mut self, edge: &TopEdge) {
        if !self.id_and_edge_by_edge_data.contains_key(&edge.data) {
            self.id_and_edge_by_edge_data.insert(edge.data.clone(), vec![]);
        }
        self.id_and_edge_by_edge_data.get_mut(&edge.data).unwrap().push(edge.clone());
    }
}

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let mut edge_map = EdgeMap::new();
    let mut tile_map = HashMap::new();
    let mut line_iter = reader.lines();
    while let Some(id_line) = line_iter.next() {
        let id_line = id_line.unwrap();
        let id: u16 = (&id_line[5..9]).parse().unwrap();
        let mut data = vec![];
        for _ in 0..10 {
            let tile_line = line_iter.next().unwrap().unwrap();
            data.push(tile_line);
        }
        line_iter.next(); // Eat the empty line
        let tile = Tile::new(id, data);
        let edges = tile.edges();
        for edge in &edges {
            edge_map.insert(edge);
        }
        tile_map.insert(id, tile);
    }

    let id_of_unmatched_edges: Vec<&TopEdge> = edge_map.id_and_edge_by_edge_data.iter()
        .filter(|&(_, ids)| ids.len() == 1)
        // .inspect(|&(edge, ids)| {
        //     if ids[0].0 == 3461 || ids[0].0 == 2287 || ids[0].0 == 3083 || ids[0].0 == 3433 {
        //         println!("Unmatched edge for {:?}: {}", ids[0], edge)
        //     }
        // })
        .flat_map(|(_, edges)| edges)
        .collect();
    let mut num_unmatched_edges_by_id = HashMap::new();
    for &edge in &id_of_unmatched_edges {
        if !num_unmatched_edges_by_id.contains_key(&edge.id) {
            num_unmatched_edges_by_id.insert(edge.id, 0);
        }
        let count = num_unmatched_edges_by_id.get(&edge.id).unwrap() + 1;
        num_unmatched_edges_by_id.insert(edge.id, count);
    }

    let part1: usize = num_unmatched_edges_by_id.iter()
        .filter(|&(_, count)| *count == 4)
        .map(|(id, _)| *id as usize)
        // .inspect(|id| println!("Corner id: {}", id))
        .product();
    println!("{}", part1);
    // 3461, 2287, 3083, 3433

    // println!();
    // println!();
    // tile_map.get(&3461).unwrap().print_data();
    // println!();
    // println!();
    // tile_map.get(&2287).unwrap().print_data();
    // println!();
    // println!();
    // tile_map.get(&3083).unwrap().print_data();
    // println!();
    // println!();
    // tile_map.get(&3433).unwrap().print_data();


    // Tile with id 3461 has no match for edge at rotation 0 (top) and 270 (left)
    let top_left_id = 3461u16;
    let mut board = [
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None, None, None, None, None],
    ];
    let top_left_tile = tile_map.remove(&top_left_id).unwrap();

    // for row in &top_left_tile.data {
    //     for c in row.chars() {
    //         print!("{}", c);
    //     }
    //     println!();
    // }
    // println!();
    // println!();
    //
    // let top_left_subgrid = top_left_tile.transformed_sea_subgrid(&Reflection::None, &Rotation::Ninety);
    // for row in &top_left_subgrid {
    //     for pix in row {
    //         print!("{}", if *pix { '#' } else { '.' });
    //     }
    //     println!();
    // }
    
    board[0][0] = Some((top_left_tile, Rotation::Zero, Reflection::None));
    // println!("Tile {} inserted at 0,0, Rotation::Zero, Reflection::None", top_left_id);

    for y in 0..12 {
        for x in 0..12 {
            if x == 0 && y == 0 {
                continue;
            }
            // println!("{},{}", x, y);
            let (prev_tile, prev_rot, prev_refl) = if x == 0 {
                board[y-1][x].as_ref().unwrap()
            } else {
                board[y][x-1].as_ref().unwrap()
            };
            // println!("Prev tile: {}, {:?}, {:?}", prev_tile.id, prev_rot, prev_refl);
            // prev_tile.print_data();
            let edge_data = if x == 0 {
                prev_tile.edge(Side::Bottom, prev_rot, prev_refl)
            } else {
                prev_tile.edge(Side::Right, prev_rot, prev_refl)
            };
            // println!("{:?}", prev_tile.edges());
            // println!("Searching for edge: {}", edge_data);
            let matching_edges: Vec<&TopEdge> = edge_map.id_and_edge_by_edge_data
                .get(&edge_data)
                .expect("Could not find ids-and-edges for edge data")
                .iter()
                .filter(|&edge| edge.id != prev_tile.id)
                .collect();
            // println!("Matched: {:?}", matching_edges);
            let edge = matching_edges.iter().next().expect("Could not find a matching edge for previous tile");
            let tile = tile_map.remove(&edge.id).expect("Could not find tile - already placed");
            let (tile_rot, tile_refl) = if x == 0 {
                // Left hand edge tiles look at bottom edge of the tile above them, so whatever
                // transform is needed to get the matching edge to be the top edge is what we want
                // to apply to the whole tile
                (edge.rotation.clone(), edge.reflection.clone())
            } else {
                match edge.reflection {
                    Reflection::None => match edge.rotation {
                        // It's natively the top edge, so need to flip X, then one rotation anticlockwise
                        Rotation::Zero => (Rotation::Ninety, Reflection::Reversed),
                        // It's natively the right edge, so just a flip
                        Rotation::Ninety => (Rotation::Zero, Reflection::Reversed), // TICK
                        // It's reversed at the bottom, so flip X, then one clockwise (== 3 anticlock)
                        Rotation::OneEighty => (Rotation::TwoSeventy, Reflection::Reversed), // TICK
                        // It's upside down at the left, so flip X then 180
                        Rotation::TwoSeventy => (Rotation::OneEighty, Reflection::Reversed),
                    },
                    Reflection::Reversed => match edge.rotation {
                        Rotation::Zero => (Rotation::Ninety, Reflection::None), // TICK
                        Rotation::Ninety => (Rotation::Zero, Reflection::None), // TICK
                        Rotation::OneEighty => (Rotation::TwoSeventy, Reflection::None), // TICK
                        Rotation::TwoSeventy => (Rotation::OneEighty, Reflection::None), // TICK
                    }
                }
            };
            // println!("Tile {} inserted at {},{} {:?}, {:?}", tile.id, x, y, tile_rot, tile_refl);
            board[y][x] = Some((tile, tile_rot, tile_refl));
        }
    }

    let mut sea = [[false; 8*12]; 8*12];
    let mut hash_count = 0;
    for tile_y in 0..12 {
        for tile_x in 0..12 {
            let (tile, rot, refl) = (board[tile_y][tile_x]).as_ref().unwrap();
            let subgrid = tile.transformed_sea_subgrid(refl, rot);
            for y in 0..8 {
                for x in 0..8 {
                    sea[tile_y*8 + y][tile_x*8 + x] = subgrid[y][x];
                    if subgrid[y][x] {
                        hash_count += 1;
                    }
                }
            }
        }
    }

    let monster_parts = 15;
    let count = count_sea_monsters(&sea);
    println!("{} - ({} * {}) = {}", hash_count, count, monster_parts, hash_count - (count * monster_parts));
    let sea_90 = rotate_sea(&sea);
    let count = count_sea_monsters(&sea_90);
    println!("{} - ({} * {}) = {}", hash_count, count, monster_parts, hash_count - (count * monster_parts));
    let sea_180 = rotate_sea(&sea_90);
    let count = count_sea_monsters(&sea_180);
    println!("{} - ({} * {}) = {}", hash_count, count, monster_parts, hash_count - (count * monster_parts));
    let sea_270 = rotate_sea(&sea_180);
    let count = count_sea_monsters(&sea_270);
    println!("{} - ({} * {}) = {}", hash_count, count, monster_parts, hash_count - (count * monster_parts));

    let reflected_sea = reflect_sea(&sea);
    let count = count_sea_monsters(&reflected_sea);
    println!("{} - ({} * {}) = {}", hash_count, count, monster_parts, hash_count - (count * monster_parts));
    let reflected_sea_90 = rotate_sea(&reflected_sea);
    let count = count_sea_monsters(&reflected_sea_90);
    println!("{} - ({} * {}) = {}", hash_count, count, monster_parts, hash_count - (count * monster_parts));
    let reflected_sea_180 = rotate_sea(&reflected_sea_90);
    let count = count_sea_monsters(&reflected_sea_180);
    println!("{} - ({} * {}) = {}", hash_count, count, monster_parts, hash_count - (count * monster_parts));
    let reflected_sea_270 = rotate_sea(&reflected_sea_180);
    let count = count_sea_monsters(&reflected_sea_270);
    println!("{} - ({} * {}) = {}", hash_count, count, monster_parts, hash_count - (count * monster_parts));
}

fn reflect_sea(sea: &[[bool; 96]; 96]) -> [[bool; 96]; 96] {
    let mut new_sea = [[false; 96]; 96];

    for y in 0..96 {
        for x in 0..96 {
            new_sea[y][x] = sea[y][95-x];
        }
    }

    new_sea
}

fn rotate_sea(sea: &[[bool; 96]; 96]) -> [[bool; 96]; 96] {
    let mut new_sea = [[false; 96]; 96];

    for y in 0..96 {
        for x in 0..96 {
            //x, y -> (y, 95 - x)
            new_sea[y][x] = sea[x][95-y];
        }
    }

    new_sea
}

fn count_sea_monsters(sea: &[[bool; 96]; 96]) -> usize {
    // println!();
    // println!();
    // for row in sea {
    //     for pix in row {
    //         print!("{}", if *pix { '#' } else { '.' });
    //     }
    //     println!();
    // }
    // println!();
    // println!();

    /*
Sea monster:
00000000001111111111
01234567890123456789
                  #
#    ##    ##    ###
 #  #  #  #  #  #
     */
    let sea_monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let sea_monster_coords = sea_monster.iter().enumerate()
        .flat_map(|(row_index, &line)| {
            line.chars().enumerate().filter_map(move |(col_index, c)| {
                if c == '#' {
                    Some((col_index, row_index))
                } else {
                    None
                }
            })
        }).collect::<Vec<_>>();
    let sea_monster_dims = (20, 3);

    let mut monster_count = 0;
    for y in 0..(96-sea_monster_dims.1) {
        for x in 0..(96-sea_monster_dims.0) {
            let mut is_monster = true;
            for (dx, dy) in &sea_monster_coords {
                let x = x + dx;
                let y = y + dy;
                if !sea[y][x] {
                    is_monster = false;
                    break;
                }
            }
            if is_monster {
                monster_count += 1;
            }
        }
    }
    monster_count
}


// 2518 - (20 * 10) = 2318 - too high
// 2518 - (20 * 15) = 2218
// 2518 - (20 * 20) = 2118 - too high, someone else's
// 2518 - (20 * 21) = 2098
// 2518 - (20 * 22) = 2078
// 2518 - (20 * 23) = 2058
// 2518 - (20 * 24) = 2038
// 2518 - (20 * 25) = 2018 - too high
// 2518 - (20 * 35) = 1818 - wrong

// 2518 - (15 * 35) = 1993
