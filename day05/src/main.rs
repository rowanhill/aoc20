#![feature(array_windows)]
#![feature(array_chunks)]

use std::io::{BufReader, BufRead};
use std::fs::File;

struct Seat {
    row_index: i32,
    seat_index: i32,
}

impl Seat {
    fn from_line(line: &String) -> Seat {
        let row_part = &line[0..7];
        let seat_part = &line[7..10];
        Seat { row_index: row_part.parse_fb_binary(), seat_index: seat_part.parse_rl_binary() }
    }

    fn id(&self) -> i32 {
        self.row_index * 8 + self.seat_index
    }
}

trait BinaryStrExt {
    fn parse_fb_binary(&self) -> i32;
    fn parse_rl_binary(&self) -> i32;
    fn parse_binary(&self, zero_char: char) -> i32;
}

impl BinaryStrExt for str {
    fn parse_fb_binary(&self) -> i32 {
        self.parse_binary('F')
    }
    fn parse_rl_binary(&self) -> i32 {
        self.parse_binary('L')
    }
    fn parse_binary(&self, zero_char: char) -> i32 {
        self.chars()
            .fold(0, |acc, c| (acc << 1) + (if c == zero_char { 0 } else { 1 }))
    }
}

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let seats: Vec<Seat> = reader.lines().map(|l| {
        let line = l.unwrap();
        Seat::from_line(&line)
    }).collect();

    let highest_id = seats.iter().fold(0, |max_so_far, seat| {
        let id = seat.id();
        if id > max_so_far { id } else { max_so_far }
    });
    println!("Part 1 {}", highest_id);

    let occupied_seats = seats.iter().fold(vec![false; 948], |mut arr, seat| {
        let id = seat.id() as usize;
        arr[id] = true;
        arr
    });
    let my_seat_id = occupied_seats.array_windows()
        .position(|&[a, b, c]| a && !b && c)
        .expect("Could not find part 2 seat") + 1;
    println!("Part 2 {}", my_seat_id);
}
