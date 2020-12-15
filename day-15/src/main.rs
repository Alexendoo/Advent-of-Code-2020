use std::collections::HashMap;

fn solve(n: usize) -> u32 {
    let input = [0, 8, 15, 2, 12, 1, 4];
    let mut spoken: HashMap<u32, (u32, Option<u32>)> = input[..input.len() - 1]
        .iter()
        .enumerate()
        .map(|(turn, &n)| (n, (turn as u32, None)))
        .collect();
    let mut last = *input.last().unwrap();

    for turn in spoken.len()..(n - 1) {
        let turn = turn as u32;
        let entry = *spoken
            .entry(last)
            .and_modify(|e| *e = (turn, Some(e.0)))
            .or_insert((turn, None));

        last = match entry {
            (prev, Some(penultimate)) => prev - penultimate,
            _ => 0,
        };
    }

    last
}

fn main() {
    println!("Part 1: {}", solve(2020));
    println!("Part 2: {}", solve(30000000));
}
