
fn main() {
    let min_time = 1008833;
    let bus_def = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,643,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,13,x,x,x,x,23,x,x,x,x,x,x,x,509,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29";

    // Example input:
    // let min_time = 939;
    // let bus_def = "7,13,x,x,59,x,31,19";

    let busses: Vec<i32> = bus_def.split(",")
        .map(|s| s.parse::<i32>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();

    let (min_delay_id, min_delay) = busses.iter()
        .map(|&id| (id, (((min_time / id) + 1) * id) - min_time))
        .min_by(|(_a,a),(_b,b)| a.cmp(b))
        .unwrap();

    println!("Part 1 {}x{} = {}", min_delay_id, min_delay, min_delay_id * min_delay);

    let mut div_rems: Vec<(u64, u64)> = bus_def.split(",")
        .map(|s| s.parse::<u64>())
        .enumerate()
        .filter(|(_, r)| r.is_ok())
        .map(|(remainder, id)| (id.unwrap(), remainder as u64))
        .collect();
    div_rems.sort_by(|(a, _a), (b, _b)| b.cmp(a));
    println!("{:?}", div_rems);

    let mut iter = div_rems.iter();
    let first = iter.next().unwrap();
    let mut n = (first.0 - (first.1 % first.0)) % first.0;
    let mut step_factor = first.0;
    println!("N % {} = {}", first.0, n);
    while let Some(cur) = iter.next() {
        let target = (cur.0 - (cur.1 % cur.0)) % cur.0;
        println!("N % {} = {}", cur.0, target);
        println!("  Step {}, n {}", step_factor, n);
        while n % cur.0 != target {
            n += step_factor;
        }
        println!("  >> {} % {} = {}", n, cur.0, target);
        step_factor *= cur.0;
    }
    println!("Part 2: {}", n);
}
