use derive_more::{Add, AddAssign};
use std::collections::HashSet;

#[derive(Add, AddAssign, Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn neighbours(self) -> impl Iterator<Item = Coord> {
        NEIGHBOURS
            .iter()
            .copied()
            .map(move |neighbour| self + neighbour)
    }

    fn nearby(self, flipped: &HashSet<Self>) -> usize {
        self.neighbours()
            .filter(|neighbour| flipped.contains(&neighbour))
            .count()
    }
}

const NEIGHBOURS: [Coord; 6] = [
    Coord::new(1, -1, 0),
    Coord::new(0, -1, 1),
    Coord::new(-1, 0, 1),
    Coord::new(-1, 1, 0),
    Coord::new(0, 1, -1),
    Coord::new(1, 0, -1),
];

fn flip(coord: Coord, set: &mut HashSet<Coord>) {
    if !set.insert(coord) {
        set.remove(&coord);
    }
}

fn main() {
    let to_flip = include_str!("input").lines().map(|line| {
        let mut cursor = 0;
        let mut coord = Coord::new(0, 0, 0);

        loop {
            let (advance, direction) = match line[cursor..].as_bytes() {
                [b'e', ..] => (1, NEIGHBOURS[0]),
                [b's', b'e', ..] => (2, NEIGHBOURS[1]),
                [b's', b'w', ..] => (2, NEIGHBOURS[2]),
                [b'w', ..] => (1, NEIGHBOURS[3]),
                [b'n', b'w', ..] => (2, NEIGHBOURS[4]),
                [b'n', b'e', ..] => (2, NEIGHBOURS[5]),
                _ => return coord,
            };

            cursor += advance;
            coord += direction;
        }
    });

    let mut flipped = HashSet::new();
    for tile in to_flip {
        flip(tile, &mut flipped);
    }

    println!("Part 1: {}", flipped.len());

    for _ in 0..100 {
        let mut next = flipped.clone();

        flipped
            .iter()
            .filter(|&&tile| {
                let nearby = tile.nearby(&flipped);

                nearby == 0 || nearby > 2
            })
            .for_each(|tile| {
                next.remove(tile);
            });

        next.extend(
            flipped
                .iter()
                .copied()
                .flat_map(Coord::neighbours)
                .filter(|tile| {
                    let nearby = tile.nearby(&flipped);

                    nearby == 2
                }),
        );

        flipped = next;
    }

    println!("Part 2: {}", flipped.len());
}
