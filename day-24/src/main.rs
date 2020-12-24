use std::collections::HashSet;

use derive_more::{Add, Sum};
#[derive(Add, Sum, Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

fn parse() -> impl Iterator<Item = Vec<Coord>> {
    let input = include_str!("input");

    input.lines().map(|line| {
        let mut cursor = 0;
        let mut out = Vec::new();

        loop {
            let (advance, coord) = match line[cursor..].as_bytes() {
                [b'e', ..] => (1, Coord::new(1, -1, 0)),
                [b's', b'e', ..] => (2, Coord::new(0, -1, 1)),
                [b's', b'w', ..] => (2, Coord::new(-1, 0, 1)),
                [b'w', ..] => (1, Coord::new(-1, 1, 0)),
                [b'n', b'w', ..] => (2, Coord::new(0, 1, -1)),
                [b'n', b'e', ..] => (2, Coord::new(1, 0, -1)),
                _ => return out,
            };

            cursor += advance;
            out.push(coord);
        }
    })
}

fn main() {
    let directions = parse();

    let mut flipped = HashSet::new();
    for coords in directions {
        let tile: Coord = coords.into_iter().sum();

        if !flipped.insert(tile) {
            flipped.remove(&tile);
        }
    }

    eprintln!("Part 1: {}", flipped.len());
}
