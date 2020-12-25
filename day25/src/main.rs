fn main() {
    let door_pub_key = 14788856u128;
    let card_pub_key = 19316454u128;

    let mut door_loop_size = None;
    let mut card_loop_size = None;
    let mut value = 1u128;
    let subject_number = 7u128;
    let mut loop_size = 0;
    while door_loop_size.is_none() || card_loop_size.is_none() {
        loop_size += 1;

        value = (value * subject_number) % 20201227;

        if value == door_pub_key {
            door_loop_size = Some(loop_size);
        }
        if value == card_pub_key {
            card_loop_size = Some(loop_size);
        }
    }
    println!("{:?} {:?}", door_loop_size, card_loop_size);

    let (smallest_loop_size, other_pub_key) = if door_loop_size.unwrap() < card_loop_size.unwrap() {
        (door_loop_size.unwrap(), card_pub_key)
    } else {
        (card_loop_size.unwrap(), door_pub_key)
    };
    value = 1u128;
    let subject_number = other_pub_key;
    for _ in 0..smallest_loop_size {
        value = (value * subject_number) % 20201227;
    }
    let encryption_key = value;

    println!("Encryption key: {}", encryption_key);
}
