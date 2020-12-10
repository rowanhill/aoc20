use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{HashSet, HashMap};

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));
    let mut iter = reader.lines();

    let mut adapters = HashSet::new();
    let mut max_jolts = 0;

    while let Some(Ok(line)) = iter.next() {
        let adapter_jolts: usize = line.parse().unwrap();
        adapters.insert(adapter_jolts);
        if adapter_jolts > max_jolts {
            max_jolts = adapter_jolts;
        }
    }

    let device_jolts = max_jolts + 3;

    let mut curr_jolts = 0;
    let mut distrib = [0, 0, 0];
    'outer: while curr_jolts < device_jolts {
        for i in 1..4 {
            if adapters.contains(&(curr_jolts + i)) {
                curr_jolts += i;
                distrib[i - 1] += 1;
                continue 'outer;
            }
        }
        // Didn't find a match, so assume we're at the end of the chain and add a difference of 3 to
        // get to the device jolts
        distrib[2] += 1;
        break;
    }

    println!("Part 1: {} x {} = {}", distrib[0], distrib[2], distrib[0] * distrib[2]);

    let mut arrangements_from_jolt:HashMap<usize, usize> = HashMap::new();

    let arrangements = count_arrangements(&adapters, &mut arrangements_from_jolt, device_jolts, 0);
    println!("Part 2: {}", arrangements);
}

fn count_arrangements(
    adapters: &HashSet<usize>,
    arrangements_from_jolt: &mut HashMap<usize, usize>,
    device_jolts: usize,
    jolts: usize,
) -> usize {
    let mut count = 0;
    for i in 1..4 {
        let j = jolts + i;
        if adapters.contains(&j) {
            if !arrangements_from_jolt.contains_key(&j) {
                let num = count_arrangements(
                    adapters,
                    arrangements_from_jolt,
                    device_jolts,
                    jolts + i,
                );
                arrangements_from_jolt.insert(j, num);
            }
            count += arrangements_from_jolt[&j];
        }
        if jolts + i == device_jolts {
            count += 1;
        }
    }
    count
}
