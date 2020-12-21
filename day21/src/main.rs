#![feature(str_split_once)]

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{HashMap, HashSet};

fn main() {
    let reader = BufReader::new(File::open("input")
        .expect("Cannot open input"));

    // A map for allergen -> {candidates for ingredients containing that allergen}
    let mut possible_ingredients: HashMap<String, HashSet<String>> = HashMap::new();
    // A map of ingredient -> number of lines that ingredient appears on
    let mut ingredients_counts: HashMap<String, usize> = HashMap::new();
    // Union of all ingredients across all lines
    let mut all_ingredients: HashSet<String> = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (ingredients, allergens) = line.split_once(" (contains ").unwrap();
        let allergens = &allergens[0..allergens.len()-1];

        let allergens = allergens.split(", ").collect::<Vec<_>>();
        let ingredients = ingredients.split(" ").map(|s| s.to_string()).collect::<HashSet<_>>();

        for allergen in allergens {
            possible_ingredients.entry(allergen.to_string())
                .or_insert(ingredients.clone())
                .retain(|i| ingredients.contains(i));
        }

        for ingredient in &ingredients {
            all_ingredients.insert(ingredient.to_string());

            *ingredients_counts.entry(ingredient.to_string())
                .or_insert(0) += 1;
        }

        // println!("{:?}", ingredients);
        // println!("{:?}", possible_ingredients);
        // println!("{:?}", ingredients_counts);
        // println!();
    }

    let mut single_allergen_ingredient_queue = possible_ingredients.iter()
        .filter(|&(_, is)| is.len() == 1)
        .map(|(_, is)| is.iter().next().unwrap().clone())
        .collect::<Vec<_>>();

    while let Some(ingredient) = single_allergen_ingredient_queue.pop() {
        for (_, is) in possible_ingredients.iter_mut() {
            if is.len() == 1 {
                // Don't remove if the allergen already has only one ingredient!
                continue;
            }
            is.remove(&ingredient);
            if is.len() == 1 {
                single_allergen_ingredient_queue.push(is.iter().next().unwrap().clone());
            }
        }
    }

    // println!("{:?}", possible_ingredients);

    let allergenic_ingredients = possible_ingredients.iter()
        .filter(|(_, is)| is.len() == 1)
        .map(|(_, is)| is.iter().next().unwrap().clone())
        .collect::<HashSet<_>>();

    let safe_ingredients_appearance_count = all_ingredients
        .difference(&allergenic_ingredients)
        .map(|i| ingredients_counts.get(i).unwrap())
        .sum::<usize>();
    println!("Part 1: {}", safe_ingredients_appearance_count);

    let mut danger_pairs = possible_ingredients.iter()
        .filter(|(_, is)| is.len() == 1)
        .map(|(a, is)| (a, is.iter().next().unwrap().clone()))
        .collect::<Vec<_>>();
    danger_pairs.sort_by(|(a1, _), (a2,_)| a1.cmp(a2));
    let canonical = danger_pairs.into_iter()
        .map(|(_, i)| i)
        .collect::<Vec<_>>()
        .join(",");
    println!("Part 2: {}", canonical);
}
