use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Food {
    ingredients: HashSet<&'static str>,
    allergens: HashSet<&'static str>,
}

fn parse() -> Vec<Food> {
    let input = include_str!("input");

    input
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line
                .strip_suffix(')')
                .unwrap()
                .split(" (contains ")
                .collect_tuple()
                .unwrap();

            Food {
                ingredients: ingredients.split(' ').collect(),
                allergens: allergens.split(", ").collect(),
            }
        })
        .collect()
}

fn main() {
    let foods = parse();

    let mut ingredients: HashMap<&str, usize> = HashMap::new();
    let mut allergens: HashMap<&str, HashSet<&str>> = HashMap::new();

    for food in foods {
        for ingredient in &food.ingredients {
            *ingredients.entry(ingredient).or_default() += 1;
        }

        for allergen in &food.allergens {
            allergens
                .entry(allergen)
                .and_modify(|set| {
                    *set = set.intersection(&food.ingredients).copied().collect();
                })
                .or_insert_with(|| food.ingredients.clone());
        }
    }

    let possible_allergens: HashSet<&str> = allergens.values().flatten().copied().collect();

    let sum: usize = ingredients
        .iter()
        .filter(|&(ingredient, _)| !possible_allergens.contains(ingredient))
        .map(|(_, count)| count)
        .sum();

    println!("Part 1: {}", sum);
}
