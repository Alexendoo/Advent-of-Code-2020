use itertools::iproduct;
use std::collections::HashSet;
use std::ops::Range;

type Coord = (i32, i32, i32, i32);
type Extents = [Range<i32>; 4];

fn each_coord([xs, ys, zs, ws]: Extents) -> impl Iterator<Item = Coord> {
    iproduct!(xs, ys, zs, ws)
}

fn surrounding_coords(centre: Coord, three_d: bool) -> impl Iterator<Item = Coord> {
    let (x, y, z, w) = centre;
    let mut extents = [
        (x - 1)..(x + 2),
        (y - 1)..(y + 2),
        (z - 1)..(z + 2),
        (w - 1)..(w + 2),
    ];

    if three_d {
        extents[3] = 0..1;
    }

    each_coord(extents).filter(move |&coord| coord != centre)
}

fn candidates(space: &HashSet<Coord>, three_d: bool) -> HashSet<Coord> {
    let mut base = space.clone();

    base.extend(
        space
            .iter()
            .flat_map(|&coord| surrounding_coords(coord, three_d))
    );

    base
}

fn solve(three_d: bool) -> usize {
    let input = include_str!("input");

    let mut space: HashSet<Coord> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(x, _)| (x as i32, y as i32, 0, 0))
        })
        .collect();

    for _ in 0..6 {
        let mut next_space = space.clone();

        for coord in candidates(&space, three_d) {
            let active = space.contains(&coord);
            let active_neighbours = surrounding_coords(coord, three_d)
                .filter(|neighbour| space.contains(neighbour))
                .count();

            match (active, active_neighbours) {
                (true, 2..=3) => {}
                (true, _) => {
                    next_space.remove(&coord);
                }
                (false, 3) => {
                    next_space.insert(coord);
                }
                (false, _) => {}
            };
        }

        space = next_space;
    }

    space.len()
}

fn main() {
    let s = std::time::Instant::now();
    println!("Part 1: {}", solve(true));
    println!("Part 2: {}", solve(false));
    println!("Elapsed: {}ms", s.elapsed().as_millis());
}
