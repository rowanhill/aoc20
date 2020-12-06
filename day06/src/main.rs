use std::fs;

fn main() {
    let input = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    let mut part1 = 0;
    let mut part2 = 0;
    input.split("\n\n").for_each(|batch| {
        let mut any_yes = [false; 26];
        let mut all_yes = [true; 26];
        batch.lines().for_each(|line| {
            let mut line_yes = [false; 26];
            line.bytes().for_each(|byte| {
                let index = (byte - 97) as usize; // 97 is 'a'
                if !any_yes[index] {
                    part1 += 1;
                }
                any_yes[index] = true;
                line_yes[index] = true;
            });
            for (i, &line_value) in line_yes.iter().enumerate() {
                if line_value == false {
                    all_yes[i] = false;
                }
            }
        });

        for &all_value in all_yes.iter() {
            if all_value {
                part2 += 1;
            }
        }
    });

    println!("Part 1 (sum of count of any saying yes): {}", part1);
    println!("Part 2 (sum of count of all saying yes): {}", part2);
}
