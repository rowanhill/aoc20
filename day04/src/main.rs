use std::fs;
use regex;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    let hcl_re = regex::Regex::new(r"#[0-9a-f]{6,6}").unwrap();
    let ecl_re = regex::Regex::new(r"(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)").unwrap();

    let mut valid_passports = 0;
    contents.split("\n\n").for_each(|batch| {
        let mut byr = false;
        let mut iyr = false;
        let mut eyr = false;
        let mut hgt = false;
        let mut hcl = false;
        let mut ecl = false;
        let mut pid = false;
        let mut cid = false;
        batch.split_whitespace().for_each(|pair| {
            let (key, _) = split_once(pair);
            match key {
                "byr" => byr = true,
                "iyr" => iyr = true,
                "eyr" => eyr = true,
                "hgt" => hgt = true,
                "hcl" => hcl = true,
                "ecl" => ecl = true,
                "pid" => pid = true,
                "cid" => cid = true,
                _ => println!("Unexpected key: {}", key),
            };
        });
        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            valid_passports += 1;
        }
    });
    println!("Part 1: {}", valid_passports);

    valid_passports = 0;
    contents.split("\n\n").for_each(|batch| {
        let mut byr = false;
        let mut iyr = false;
        let mut eyr = false;
        let mut hgt = false;
        let mut hcl = false;
        let mut ecl = false;
        let mut pid = false;
        let mut cid = false;
        batch.split_whitespace().for_each(|pair| {
            let (key, value) = split_once(pair);
            match key {
                "byr" => byr = value.len() == 4 && value.parse::<i32>().unwrap() >= 1920 && value.parse::<i32>().unwrap() <= 2002,
                "iyr" => iyr = value.len() == 4 && value.parse::<i32>().unwrap() >= 2010 && value.parse::<i32>().unwrap() <= 2020,
                "eyr" => eyr = value.len() == 4 && value.parse::<i32>().unwrap() >= 2020 && value.parse::<i32>().unwrap() <= 2030,
                "hgt" => {
                    if value.ends_with("cm") {
                        let cm = value.replace("cm", "").parse::<i32>().unwrap();
                        hgt = cm >= 150 && cm <= 193;
                    } else if value.ends_with("in") {
                        let cm = value.replace("in", "").parse::<i32>().unwrap();
                        hgt = cm >= 59 && cm <= 76;
                    }
                },
                "hcl" => hcl = hcl_re.is_match(value),
                "ecl" => ecl = ecl_re.is_match(value),
                "pid" => pid = value.len() == 9 && value.parse::<i32>().is_ok(),
                "cid" => cid = true,
                _ => println!("Unexpected key: {}", key),
            };
        });
        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            valid_passports += 1;
        }
    });
    println!("Part 2: {}", valid_passports);
}

fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ':');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}
