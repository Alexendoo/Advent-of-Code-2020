use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Mask(u64, u64, u64),
    Mem(u64, u64),
}

fn parse() -> Vec<Op> {
    include_str!("input")
        .lines()
        .map(|line| {
            let mut parts = line
                .split(|ch: char| !ch.is_ascii_alphanumeric())
                .filter(|part| !part.is_empty());

            match parts.next()? {
                "mask" => {
                    let mask = parts.next()?;

                    let mut zero = !0;
                    let mut one = 0;
                    let mut x = !0;

                    for (i, ch) in mask.chars().rev().enumerate() {
                        match ch {
                            '0' => zero &= !(1 << i),
                            '1' => one |= 1 << i,
                            'X' => x &= !(1 << i),
                            _ => unreachable!(),
                        }
                    }

                    Some(Op::Mask(zero, one, x))
                }
                "mem" => Some(Op::Mem(
                    parts.next()?.parse().unwrap(),
                    parts.next()?.parse().unwrap(),
                )),
                _ => unreachable!(),
            }
        })
        .collect::<Option<Vec<Op>>>()
        .unwrap()
}

fn part_1(ops: &[Op]) {
    let mut memory = HashMap::<u64, u64>::new();

    let mut zeros_mask = 0;
    let mut ones_mask = 0;

    for op in ops {
        match *op {
            Op::Mask(zeroes, ones, _) => {
                zeros_mask = zeroes;
                ones_mask = ones;
            }
            Op::Mem(index, value) => {
                memory.insert(index, (value & zeros_mask) | ones_mask);
            }
        }
    }

    println!("Part 1: {}", memory.values().sum::<u64>());
}

fn part_2(ops: &[Op]) {
    let mut memory = HashMap::<u64, u64>::new();

    let mut ones_mask = 0;
    let mut x_mask = 0;

    for op in ops {
        match *op {
            Op::Mask(_, ones, xs) => {
                ones_mask = ones;
                x_mask = xs;
            }
            Op::Mem(mut index, value) => {
                index |= ones_mask;
                index |= !x_mask;

                let mut index_mask = x_mask;

                for _ in 0..(1 << x_mask.count_zeros()) {
                    index_mask = x_mask | index_mask.overflowing_add(1).0;

                    memory.insert(index & index_mask, value);
                }
            }
        }
    }

    println!("Part 2: {}", memory.values().sum::<u64>());
}

fn main() {
    let ops = parse();

    part_1(&ops);
    part_2(&ops);
}
