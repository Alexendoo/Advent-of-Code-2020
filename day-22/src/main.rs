use itertools::Itertools;
use std::collections::VecDeque;

fn parse() -> (VecDeque<usize>, VecDeque<usize>) {
    let input = include_str!("input");

    let (p1, p2) = input.split("\n\n").collect_tuple().unwrap();

    let ints = |p: &str| p.lines().skip(1).map(str::parse).try_collect().unwrap();

    (ints(p1), ints(p2))
}

fn main() {
    let (mut player1, mut player2) = parse();

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        if card1 > card2 {
            player1.extend(&[card1, card2]);
        } else {
            player2.extend(&[card2, card1]);
        }
    }

    let score = player1
        .max(player2)
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (i + 1) * card);

    println!("Part 1: {}", score);
}
