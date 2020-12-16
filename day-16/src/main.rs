use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use itertools::Itertools;

fn parse_range(r: &str) -> RangeInclusive<u32> {
    let (start, end) = r
        .split('-')
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap();

    start..=end
}

struct Input {
    rules: Vec<(&'static str, RangeInclusive<u32>, RangeInclusive<u32>)>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}

fn parse() -> Input {
    let input = include_str!("input");

    let mut parts = input.split("\n\n");
    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (name, ranges) = line.split(": ").collect_tuple().unwrap();
            let (a, b) = ranges.split(" or ").collect_tuple().unwrap();

            (name, parse_range(a), parse_range(b))
        })
        .collect_vec();

    let mut tickets = || {
        parts.next().unwrap().lines().skip(1).map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
        })
    };

    let my_ticket = tickets().next().unwrap();

    let nearby_tickets = tickets().collect_vec();

    Input {
        rules,
        my_ticket,
        nearby_tickets,
    }
}

fn part_1(input: &Input) {
    let error_rate: u32 = input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|field| {
            !input
                .rules
                .iter()
                .any(|rule| rule.1.contains(field) || rule.2.contains(field))
        })
        .sum();

    println!("Part 1: {}", error_rate);
}

fn part_2(input: Input) {
    let Input {
        rules,
        my_ticket,
        nearby_tickets,
    } = input;

    let valid_tickets = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket.iter().all(|field| {
                rules
                    .iter()
                    .any(|rule| rule.1.contains(field) || rule.2.contains(field))
            })
        })
        .collect_vec();

    let mut possible_cols = rules
        .iter()
        .map(|rule| {
            (0..my_ticket.len())
                .filter(|&column| {
                    valid_tickets.iter().all(|ticket| {
                        let field = &ticket[column as usize];

                        rule.1.contains(field) || rule.2.contains(field)
                    })
                })
                .collect::<HashSet<_>>()
        })
        .collect_vec();

    let mut found = HashMap::new();

    for _ in 0..rules.len() {
        let (i, set) = possible_cols
            .iter()
            .find_position(|set| set.len() == 1)
            .unwrap();
        let col = *set.iter().next().unwrap();
        found.insert(i, col);

        for cols in &mut possible_cols {
            cols.remove(&col);
        }
    }

    let product: u64 = (0..6).map(|i| my_ticket[found[&i]] as u64).product();

    println!("Part 2: {}", product);
}

fn main() {
    let input = parse();

    part_1(&input);
    part_2(input);
}
