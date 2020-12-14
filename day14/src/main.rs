#![feature(str_split_once)]

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    let mut or_mask = 0u64;
    let mut and_mask = 0u64;
    let mut mem = [0u64; 100000];
    let mut mem2: HashMap<u64, u64> = HashMap::new();
    let mut floating_bits: Vec<usize> = vec![];
    for line in reader.lines() {
        let line = line.expect("Could not read line");

        if line.starts_with("mask = ") {
            or_mask = 0;
            and_mask = 0;
            floating_bits = vec![];
            for (i, c) in (&line[7..]).chars().enumerate() {
                or_mask *= 2;
                and_mask *= 2;
                match c {
                    'X' => {
                        and_mask += 1; // don't want to force a 0
                        //or_mask takes 0 - don't want to force a 1
                        floating_bits.push(35 - i);
                    },
                    '0' => {
                        //and_mask takes 0 - want to force a 0
                        //or_mask takes 0 - don't want to force a 1
                    },
                    '1' => {
                        and_mask += 1; // don't want to force a 0
                        or_mask += 1; // want to force a 1
                    },
                    _ => panic!("Unexpected mask character {}", c)
                }
            }
        } else if line.starts_with("mem[") {
            let (index, value) = (&line[4..]).split_once("] = ")
                .expect("Could not split mem line");
            let index = index.parse::<usize>().expect("Could not parse mem index");
            let value = value.parse::<u64>().expect("Could not parse mem value");

            // Part 1
            let masked_value = (value & and_mask) | or_mask;
            mem[index] = masked_value;

            // Part 2

            // Take the index, set the 1s from the mask, and set floating bits to 0 to start with
            let floating_zeros_mask = floating_bits.iter()
                .fold(0b111111111111111111111111111111111111u64, |acc, i| {
                    acc ^ 1 << i
                });
            let masked_index = (index as u64 | or_mask) & floating_zeros_mask;

            // Loop over all possible values of the floating bits - there are 2^[num floating bits]
            // such values
            for bit_values in 0..2u64.pow(floating_bits.len() as u32) {
                let mut float_masked_index = masked_index;
                // Set each floating bit to its current value from bit_values
                for (i, floating_bit) in floating_bits.iter().enumerate() {
                    // If the floating bit's value (in bit_values) is 0, we need to clear the bit in
                    // the memory index; otherwise we need to set the bit.
                    let should_clear_bit = bit_values & (1 << i) == 0;
                    if should_clear_bit {
                        float_masked_index &= 0b111111111111111111111111111111111111u64 ^ 1 << floating_bit;
                    } else {
                        float_masked_index |= 1u64 << floating_bit
                    };
                }
                mem2.insert(float_masked_index, value);
            }
        } else {
            panic!("Unexpected line format {}", line);
        }
    }

    println!("{}", mem.iter().sum::<u64>());
    println!("{}", mem2.iter().map(|(_, v)| *v).sum::<u64>());
}
