use itertools::iproduct;
use std::collections::HashSet;
use std::ops::Range;

type Coord = (i32, i32, i32);
type Extents = [Range<i32>; 3];

fn expand_extents(extents: &mut Extents) {
    for range in extents {
        range.start -= 1;
        range.end += 1;
    }
}

fn each_coord([xs, ys, zs]: Extents) -> impl Iterator<Item = Coord> {
    iproduct!(xs, ys, zs)
}

fn surrounding_coords(centre: Coord) -> impl Iterator<Item = Coord> {
    let (x, y, z) = centre;
    let extents = [(x - 1)..(x + 2), (y - 1)..(y + 2), (z - 1)..(z + 2)];

    each_coord(extents).filter(move |&coord| coord != centre)
}

fn main() {
    let input = include_str!("input");

    let width = input.find("\n").unwrap() as i32;
    let height = input.lines().count() as i32;

    let mut extents = [0..width, 0..height, 0..1];

    let mut space: HashSet<Coord> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .map(move |(x, _)| (x as i32, y as i32, 0))
        })
        .collect();

    for _ in 0..6 {
        expand_extents(&mut extents);
        let mut next_space = space.clone();

        for coord in each_coord(extents.clone()) {
            let active = space.contains(&coord);
            let active_neighbours = surrounding_coords(coord)
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

    println!("Part 1: {}", space.len());
}
