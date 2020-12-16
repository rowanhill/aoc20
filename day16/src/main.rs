use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

/*
your ticket:
73,101,67,97,149,53,89,113,79,131,71,127,137,61,139,103,83,107,109,59
 */

fn main() {
    let rule_names = [
        "departure location".to_string(),
        "departure station".to_string(),
        "departure platform".to_string(),
        "departure track".to_string(),
        "departure date".to_string(),
        "departure time".to_string(),
        "arrival location".to_string(),
        "arrival station".to_string(),
        "arrival platform".to_string(),
        "arrival track".to_string(),
        "class".to_string(),
        "duration".to_string(),
        "price".to_string(),
        "route".to_string(),
        "row".to_string(),
        "seat".to_string(),
        "train".to_string(),
        "type".to_string(),
        "wagon".to_string(),
        "zone".to_string(),
    ]; 
    let mut rules = HashMap::new();
    rules.insert("departure location".to_string(), (26..715+1, 727..972+1));
    rules.insert("departure station".to_string(), (45..164+1, 175..960+1));
    rules.insert("departure platform".to_string(), (43..247+1, 270..972+1));
    rules.insert("departure track".to_string(), (25..306+1, 330..949+1));
    rules.insert("departure date".to_string(), (26..635+1, 660..961+1));
    rules.insert("departure time".to_string(), (42..773+1, 793..961+1));
    rules.insert("arrival location".to_string(), (28..928+1, 943..952+1));
    rules.insert("arrival station".to_string(), (36..593+1, 613..966+1));
    rules.insert("arrival platform".to_string(), (33..280+1, 297..951+1));
    rules.insert("arrival track".to_string(), (44..358+1, 371..974+1));
    rules.insert("class".to_string(), (39..815+1, 839..955+1));
    rules.insert("duration".to_string(), (39..573+1, 589..959+1));
    rules.insert("price".to_string(), (49..846+1, 865..962+1));
    rules.insert("route".to_string(), (30..913+1, 924..954+1));
    rules.insert("row".to_string(), (29..865+1, 890..965+1));
    rules.insert("seat".to_string(), (44..667+1, 683..969+1));
    rules.insert("train".to_string(), (32..473+1, 482..969+1));
    rules.insert("type".to_string(), (40..424+1, 432..953+1));
    rules.insert("wagon".to_string(), (49..156+1, 164..960+1));
    rules.insert("zone".to_string(), (34..521+1, 534..971+1));

    let reader = BufReader::new(File::open("nearby-tickets")
        .expect("Cannot open input"));

    let mut valid_field_values: [Vec<usize>; 20] = [
        vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![],
        vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]
    ];

    let mut error_rate = 0;
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let field_values: Vec<usize> = line.split(",")
            .map(|n| n.parse().expect("Could not parse field value"))
            .collect();

        let mut is_valid = true;
        for field_value in &field_values {
            let mut field_value_valid = false;
            'inner: for (a, b) in rules.values() {
                if a.contains(field_value) || b.contains(field_value) {
                    field_value_valid = true;
                    break 'inner;
                }
            }
            if !field_value_valid {
                is_valid = false;
                error_rate += field_value;
                continue;
            }
        }

        if is_valid {
            let mut i = 0;
            for field_value in &field_values {
                valid_field_values[i].push(*field_value);
                i += 1;
            }
        }
    }
    println!("Part 1: {}", error_rate);

    let my_ticket = [73,101,67,97,149,53,89,113,79,131,71,127,137,61,139,103,83,107,109,59];
    for (i, v) in my_ticket.iter().enumerate() {
        valid_field_values[i].push(*v);
    }

    for (_, rule_name) in rule_names.iter().enumerate() {
        let mut possible_fields = [true; 20];
        let (a, b) = rules.get(rule_name).expect("Could not find rule");

        for (field_index, valid_values) in valid_field_values.iter().enumerate() {
            for value in valid_values {
                if !a.contains(value) && !b.contains(value) {
                    possible_fields[field_index] = false;
                    // println!("Ruling out field {} for {} because {} not in {:?},{:?}", field_index, rule_name, value, a, b);
                    break;
                }
            }
        }

        let still_possible: Vec<usize> = possible_fields.iter()
            .enumerate()
            .filter(|&(_, is_possible)| *is_possible == true)
            .map(|(index, _)| index)
            .collect();
        println!("{:20}: {:?}", rule_name, still_possible);
    }

    println!("Part 2: {}",
             my_ticket[2] *
                 my_ticket[14] *
                 my_ticket[16] *
                 my_ticket[7] *
                 my_ticket[19] *
                 my_ticket[13]
    );
}

/*
arrival location    : [10]
type                : [3]                 
arrival platform    : [6]               
row                 : [9] 
zone                : [18] 
wagon               : [11]       
price               : [17]
   
departure location  : [2]
departure station   : [14]   
departure date      : [16]
departure platform  : [7]
departure track     : [19]
departure time      : [13]

class               : [15]
train               : [8]
arrival track       : [1]
duration            : [5]
arrival station     : [4]
route               : [0]
seat                : [12]
 */


