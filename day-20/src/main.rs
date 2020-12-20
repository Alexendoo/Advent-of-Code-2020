use std::collections::HashMap;

#[derive(Debug)]
struct Tile {
    id: u64,

    top: String,
    bottom: String,
    left: String,
    right: String,
}

fn parse() -> Vec<Tile> {
    let input = include_str!("input");

    let mut tiles = Vec::new();

    for tile in input.split("\n\n") {
        let id = tile[5..9].parse().unwrap();
        let top = tile[11..21].to_string();
        let bottom = tile[110..120].to_string();

        let mut left = String::with_capacity(10);
        let mut right = String::with_capacity(10);

        for line in tile.lines().skip(1) {
            left.push(line.chars().next().unwrap());
            right.push(line.chars().next_back().unwrap());
        }

        tiles.push(Tile {
            id,
            top,
            bottom,
            left,
            right,
        })
    }

    tiles
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let tiles = parse();

    let mut map: HashMap<String, Vec<_>> = HashMap::new();
    for tile in tiles {
        map.entry(rev(&tile.top)).or_default().push(tile.id);
        map.entry(rev(&tile.bottom)).or_default().push(tile.id);
        map.entry(rev(&tile.left)).or_default().push(tile.id);
        map.entry(rev(&tile.right)).or_default().push(tile.id);

        map.entry(tile.top).or_default().push(tile.id);
        map.entry(tile.bottom).or_default().push(tile.id);
        map.entry(tile.left).or_default().push(tile.id);
        map.entry(tile.right).or_default().push(tile.id);
    }

    let mut seen_sides = HashMap::<_, u8>::new();
    for id in map.values().filter(|v| v.len() == 1).map(|v| v[0]) {
        *seen_sides.entry(id).or_default() += 1;
    }

    let product = seen_sides
        .into_iter()
        .filter(|&(_, count)| count == 4)
        .fold(1, |acc, (id, _)| acc * id);

    println!("Part 1: {}", product);
}
