use itertools::Itertools;
use std::collections::HashMap;
use std::mem::take;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: u64,
    centre: String,

    top: String,
    right: String,
    bottom: String,
    left: String,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.id)?;

        writeln!(f, "top    {}", self.top)?;
        writeln!(f, "right  {}", self.right)?;
        writeln!(f, "bottom {}", self.bottom)?;
        writeln!(f, "left   {}\n", self.left)?;

        for i in 0..8 {
            writeln!(f, "{}", &self.centre[i * 8..i * 8 + 8])?;
        }

        Ok(())
    }
}

impl Tile {
    fn new(tile: &str) -> Self {
        let mut left = String::with_capacity(10);
        let mut right = String::with_capacity(10);

        let mut centre = String::with_capacity(64);

        for line in tile.lines().skip(1) {
            left.push(line.chars().next().unwrap());
            right.push(line.chars().next_back().unwrap());
        }

        for line in tile.lines().skip(2).take(8) {
            centre.push_str(&line[1..9]);
        }

        Self {
            id: tile[5..9].parse().unwrap(),
            centre,

            top: tile[11..21].to_string(),
            bottom: tile[110..120].to_string(),
            left,
            right,
        }
    }

    fn rotate(&mut self) {
        let top = take(&mut self.top);
        let right = take(&mut self.right);
        let bottom = take(&mut self.bottom);
        let left = take(&mut self.left);

        self.top = right;
        self.right = bottom;
        self.bottom = left;
        self.left = top;

        // # . .
        // . # .
        // . # .

        // . . .
        // . # #
        // # . .

        let mut centre = String::with_capacity(64);
        for col in (0..8).rev() {
            for row in 0..8 {
                let i = row * 8 + col;
                let ch = self.centre[i..].chars().next().unwrap();
                centre.push(ch);
            }
        }

        self.centre = centre;
    }
}

#[derive(Clone, Copy, Debug)]
struct Side<'a> {
    // id: u64,
    tile: &'a Tile,
    orientation: Orientation,
    flipped: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}
use Orientation::*;

fn parse() -> Vec<Tile> {
    let input = include_str!("input");

    input.split("\n\n").map(Tile::new).collect()
}

fn rev(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let tiles = parse();

    let mut sides: HashMap<String, Vec<Side>> = HashMap::new();
    for tile in &tiles {
        let mut add = |side, orientation, flipped| {
            sides.entry(side).or_default().push(Side {
                tile,
                orientation,
                flipped,
            })
        };

        add(rev(&tile.top), Top, true);
        add(rev(&tile.bottom), Bottom, true);
        add(rev(&tile.left), Left, true);
        add(rev(&tile.right), Right, true);

        add(tile.top.clone(), Top, false);
        add(tile.bottom.clone(), Bottom, false);
        add(tile.left.clone(), Left, false);
        add(tile.right.clone(), Right, false);
    }

    let mut unpaired = HashMap::<_, u8>::new();
    for side in sides.values().filter(|v| v.len() == 1).map(|v| v[0]) {
        *unpaired.entry(side.tile).or_default() += 1;
    }

    let corners: Vec<&Tile> = unpaired
        .iter()
        .filter(|(_, &count)| count == 4)
        .map(|(&tile, _)| tile)
        .collect();

    println!(
        "Part 1: {}",
        corners.iter().fold(1, |acc, tile| acc * tile.id)
    );

    let mut start: Tile = corners[0].clone();
    eprintln!("start = {:#?}", start);

    let (side1, side2) = sides
        .iter()
        .filter(|(_, s)| s.len() == 1 && s.iter().any(|&side| !side.flipped && side.tile == &start))
        .map(|(_, s)| s[0])
        .collect_tuple()
        .unwrap();

    let or1 = side1.orientation;
    let or2 = side2.orientation;
    let rotate_n = match (or1.min(or2), or1.max(or2)) {
        (Top, Right) => 0,
        (Right, Bottom) => 1,
        (Bottom, Left) => 2,
        (Top, Left) => 3,
        u => unreachable!("{:?}", u),
    };

    for _ in 0..rotate_n {
        start.rotate();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_tile() {
        let mut start = Tile::new(
            r"Tile 1111:
#.......#.
.###......
.#........
..........
..........
..........
..........
..........
..........
..........",
        );
        start.rotate();

        let end = Tile::new(
            r"Tile 1111:
..........
#.........
..........
..........
..........
..........
.#........
.#........
.##.......
#.........",
        );

        assert_eq!(start, end);
    }
}
