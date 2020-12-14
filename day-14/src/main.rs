use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Mask(u64, u64),
    Mem(usize, u64),
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

                    for (i, ch) in mask.chars().rev().enumerate() {
                        match ch {
                            '0' => zero &= !(1 << i),
                            '1' => one |= 1 << i,
                            _ => {}
                        }
                    }

                    Some(Op::Mask(zero, one))
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

fn main() {
    let ops = parse();

    let mut memory = HashMap::<usize, u64>::new();

    let mut zeros_mask = 0;
    let mut ones_mask = 0;

    for op in ops {
        match op {
            Op::Mask(zeroes, ones) => {
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
