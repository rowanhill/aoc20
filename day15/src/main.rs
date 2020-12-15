use std::collections::HashMap;

fn main() {
    let starting_numbers = [2usize,1,10,11,0,6];
    // let starting_numbers = [0,3,6];
    let mut most_recent_appearance: HashMap<usize, usize> = HashMap::new();
    let mut turn = 1;
    for num in starting_numbers.iter() {
        most_recent_appearance.insert(*num, turn);
        turn += 1;
    }

    let mut age = 0;

    while turn <= 30000000 {
        if turn == 2020 {
            println!("Part 1 {}: {}", turn, age);
        } else if turn == 30000000 {
            println!("Part 2 {}: {}", turn, age);
        }
        let old_turn = most_recent_appearance.insert(age, turn);
        age = match old_turn {
            Some(n) => turn - n,
            None => 0
        };

        turn += 1;
    }
}
