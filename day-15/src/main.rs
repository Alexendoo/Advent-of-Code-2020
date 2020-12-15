use std::collections::HashMap;

fn main() {
    let input = [0, 8, 15, 2, 12, 1, 4];

    let mut spoken: HashMap<usize, (usize, Option<usize>)> = input[..input.len() - 1]
        .iter()
        .enumerate()
        .map(|(turn, &n)| (n, (turn, None)))
        .collect();
    let mut last = *input.last().unwrap();

    for turn in spoken.len()..2019 {
        let entry = *spoken
            .entry(last)
            .and_modify(|e| *e = (turn, Some(e.0)))
            .or_insert((turn, None));

        last = match entry {
            (prev, Some(penultimate)) => prev - penultimate,
            _ => 0,
        };
    }

    println!("Part 1: {}", last);
}
