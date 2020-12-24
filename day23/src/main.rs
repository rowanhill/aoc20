use std::collections::HashMap;

fn compute_vec(mut cups: Vec<usize>, rounds: usize) -> Vec<usize> {
    let mut current_cup_index = 0;

    for _ in 0..rounds {
        let current_cup = *cups.get(current_cup_index).unwrap();

        let mut removal_index = (current_cup_index + 1) % cups.len();
        let a = cups.remove(removal_index);
        if removal_index < current_cup_index {
            current_cup_index -= 1;
        }

        removal_index = (current_cup_index + 1) % cups.len();
        let b = cups.remove(removal_index);
        if removal_index < current_cup_index {
            current_cup_index -= 1;
        }

        removal_index = (current_cup_index + 1) % cups.len();
        let c = cups.remove(removal_index);
        if removal_index < current_cup_index {
            current_cup_index -= 1;
        }

        let next_cup = *cups.get((current_cup_index + 1) % cups.len()).unwrap();
        let mut destination_cup = current_cup - 1;
        if destination_cup <= 0 {
            destination_cup += cups.len() + 3;
        }
        while destination_cup == a || destination_cup == b || destination_cup == c {
            destination_cup -= 1;
            if destination_cup <= 0 {
                destination_cup += cups.len() + 3;
            }
        }
        let destination_cup_index = cups.iter()
            .position(|cup| cup == &destination_cup)
            .expect(&format!("Could not find cup {} in {:?}", destination_cup, cups));
        cups.insert(destination_cup_index + 1, c);
        cups.insert(destination_cup_index + 1, b);
        cups.insert(destination_cup_index + 1, a);

        current_cup_index = cups.iter().position(|cup| cup == &next_cup).unwrap();
    }

    cups
}

fn compute_map(cups_vec: Vec<usize>, rounds: usize) -> HashMap<usize, usize> {
    // A map of one cup's label to the next-cup-in-the-circle's label
    let mut cups = HashMap::with_capacity(cups_vec.len());

    for i in 0..(cups_vec.len()-1) {
        cups.insert(cups_vec[i], cups_vec[i+1]);
    }
    cups.insert(cups_vec[cups_vec.len() - 1], cups_vec[0]);

    let mut current_cup = cups_vec[0];

    for _ in 0..rounds {
        // Remove three cups to the right.
        // The fourth cup will be our next 'current' cup
        let a = cups[&current_cup];
        let b = cups[&a];
        let c = cups[&b];
        let next_cup = cups[&c];
        cups.insert(current_cup, next_cup);

        // Calculate the destination cup - go down from the current cup to find one still in the
        // circle, wrapping around if needed
        let mut dest_cup = current_cup - 1;
        loop {
            // 1 is the lowest cup, so once we hit 0, wrap around to the top
            if dest_cup < 1 {
                dest_cup += cups_vec.len();
            }
            // if the candidate destination cup has been removed (i.e. is a, b, or c), try the next
            // lowest
            if dest_cup == a || dest_cup == b || dest_cup == c {
                dest_cup -= 1;
            } else {
                // If the candidate cup in the circle, we're done
                break;
            }
        }

        // Insert the (a->b->c) chain into the circle. dest->tmp becomes dest->a->b->c->tmp
        let tmp = cups[&dest_cup];
        cups.insert(dest_cup, a);
        cups.insert(c, tmp);

        current_cup = next_cup;
    }

    cups
}

fn main() {
    let mut cups = vec![5,8,3,9,7,6,2,4,1];
    cups = compute_vec(cups, 100);
    println!("Part 1: {:?}", cups);

    cups = vec![5,8,3,9,7,6,2,4,1];
    let cups_map = compute_map(cups, 100);
    // println!("{:?}", cups_map);
    print!("Part 1: ");
    let mut cup = 1;
    loop {
        cup = cups_map[&cup];
        if cup == 1 {
            break;
        }
        print!("{}", cup);
    }
    println!();

    cups = vec![5,8,3,9,7,6,2,4,1];
    for i in 10..=1_000_000 {
        cups.push(i);
    }
    let cups_map = compute_map(cups, 10_000_000);
    let next = cups_map[&1];
    let next_next = cups_map[&next];
    println!("{} x {} = {}", next, next_next, next * next_next);
}
