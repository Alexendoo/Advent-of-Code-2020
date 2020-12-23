use itertools::Itertools;
use std::collections::VecDeque;
use std::mem;

fn main() {
    let input = "362981754";

    let mut cups: VecDeque<u8> = input.bytes().map(|b| b - b'0').collect();
    cups.rotate_left(1);

    for _ in 0..100 {
        let mut pickup = cups.split_off(3);
        mem::swap(&mut cups, &mut pickup);

        let next = *cups.back().unwrap();

        let dest = cups
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, cup)| cup < next)
            .max_by_key(|&(_, cup)| cup)
            .map(|(i, _)| i)
            .or_else(|| cups.iter().position_max())
            .unwrap();

        cups.rotate_left(dest + 1);
        cups.append(&mut pickup);
        cups.rotate_right(dest + 3);
    }


    let pos_one = cups.iter().position(|&cup| cup == 1).unwrap();
    cups.rotate_left(pos_one);

    println!("Part 1: {}", cups.iter().skip(1).join(""));
}
