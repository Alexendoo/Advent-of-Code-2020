use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: u64,
    board: String,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        write!(f, "T{}/{:x}", self.id, hasher.finish())
    }
}

impl Tile {
    fn new(tile: &str) -> Self {
        Self {
            id: tile[5..9].parse().unwrap(),
            board: tile[11..].lines().map(str::trim).collect(),
        }
    }

    fn rotated(&self) -> Self {
        // # . .
        // . # .
        // . # .

        // . . .
        // . # #
        // # . .

        let mut board = String::with_capacity(100);
        for col in (0..10).rev() {
            for row in 0..10 {
                let i = row * 10 + col;
                let ch = self.board[i..].chars().next().unwrap();
                board.push(ch);
            }
        }

        assert_eq!(board.len(), self.board.len());

        Self { id: self.id, board }
    }

    fn flipped(&self) -> Self {
        let mut board = String::with_capacity(100);

        for row in (0..10).rev() {
            board.extend(self.board.chars().rev().skip(row * 10).take(10));
        }

        assert_eq!(board.len(), self.board.len());

        Self { id: self.id, board }
    }

    fn sides(&self) -> Sides {
        let board = &self.board;

        let mut left = String::with_capacity(10);
        let mut right = String::with_capacity(10);

        for i in (0..100).step_by(10) {
            left.push(board[i..].chars().next().unwrap());
            right.push(board[i + 9..].chars().next().unwrap());
        }

        Sides {
            top: board[0..10].to_string(),
            right,
            bottom: board[90..100].to_string(),
            left,
        }
    }

    fn row(&self, i: usize) -> &str {
        let start = i * 10 + 11;
        let end = start + 8;

        &self.board[start..end]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sides {
    top: String,
    right: String,
    bottom: String,
    left: String,
}

#[derive(Clone, Copy, Debug)]
struct Side<'a> {
    tile: &'a Tile,
    orientation: Orientation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

fn sqrt(u: usize) -> usize {
    (u as f64).sqrt() as usize
}

fn main() {
    let tiles = parse();

    let mut sides: HashMap<String, Vec<(Tile, Orientation)>> = HashMap::new();

    for mut tile in tiles.iter().cloned() {
        let mut add =
            |side, orientation, tile| sides.entry(side).or_default().push((tile, orientation));

        let mut add_all = |tile: &Tile| {
            let sides = tile.sides();

            add(sides.top, Top, tile.clone());
            add(sides.right, Right, tile.clone());
            add(sides.bottom, Bottom, tile.clone());
            add(sides.left, Left, tile.clone());
        };

        for _ in 0..4 {
            add_all(&tile);
            add_all(&tile.flipped());

            tile = tile.rotated();
        }
    }

    let tiles_by_id: HashMap<u64, &Tile> = tiles.iter().map(|tile| (tile.id, tile)).collect();

    let mut unpaired = HashMap::<_, u8>::new();
    for id in sides.values().filter(|v| v.len() == 4).map(|v| v[0].0.id) {
        *unpaired.entry(id).or_default() += 1;
    }

    let corners: Vec<&Tile> = unpaired
        .iter()
        .filter(|(_, &count)| count == 4)
        .map(|(&id, _)| tiles_by_id[&id])
        .collect();

    println!(
        "Part 1: {}",
        corners.iter().fold(1, |acc, tile| acc * tile.id)
    );

    eprintln!("unpaired = {:#?}", unpaired);
    eprintln!("corners = {:#?}", corners);

    let start_id = corners[0].id;

    let start = sides
        .iter()
        .map(|(_, tiles)| tiles)
        .filter(|tiles| tiles.len() == 4)
        .filter(|tiles| tiles.iter().any(|(tile, _)| tile.id == start_id))
        .flatten()
        .map(|(tile, _)| tile)
        .find(|tile| {
            let Sides { top, left, .. } = tile.sides();

            sides[&top].len() == 4 && sides[&left].len() == 4
        })
        .unwrap();

    eprintln!("start = {:#?}", start);

    let width = sqrt(tiles.len());

    let next_tile = |side, id, ori| {
        &sides[&side]
            .iter()
            .find(|t| t.0.id != id && ori == t.1)
            .unwrap()
            .0
    };
    let mut grid = vec![start];

    for _ in 1..width {
        let last = grid.last().unwrap();
        let next = next_tile(last.sides().right, last.id, Left);

        grid.push(next);
    }

    for i in width..width.pow(2) {
        let last = grid[i - width];
        let next = next_tile(last.sides().bottom, last.id, Top);

        grid.push(next);
    }
    eprintln!("grid = {:#?}", grid);
    let mut complete_board = String::new();
    for grid_row in 0..width {
        for board_row in 0..8 {
            for grid_col in 0..width {
                let grid_index = grid_row * width + grid_col;
                complete_board.push_str(&grid[grid_index].row(board_row))
            }
        }
    }

    dbg!(&complete_board);
    dbg!(complete_board.len());

    for i in 0..(8 * width) {
        let i = i * width * 8;

        println!("{}", &complete_board[i..i + 3 * 8]);
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn rotated() {
        let start = Tile::new(
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

        assert_eq!(start.rotated(), end);
    }

    #[test]
    fn flipped() {
        let start = Tile::new(
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

        let end = Tile::new(
            r"Tile 1111:
              .#.......#
              ......###.
              ........#.
              ..........
              ..........
              ..........
              ..........
              ..........
              ..........
              ..........",
        );

        assert_eq!(start.flipped(), end);
    }

    #[test]
    fn sides() {
        let tile = Tile::new(
            r"Tile 1111:
              #.......#.
              .###......
              .#........
              ..........
              ..........
              ..........
              ..........
              .........#
              .........#
              .........#",
        );

        let sides = Sides {
            top: "#.......#.".to_string(),
            right: ".......###".to_string(),
            bottom: ".........#".to_string(),
            left: "#.........".to_string(),
        };

        assert_eq!(tile.sides(), sides);
    }

    #[test]
    fn row() {
        let tile = Tile::new(
            r"Tile 1111:
              #.......#.
              .###......
              .#........
              ..........
              ..........
              ..........
              ..........
              .........#
              ..####...#
              .........#",
        );

        assert_eq!(tile.row(0), "###.....");
        assert_eq!(tile.row(1), "#.......");
        assert_eq!(tile.row(7), ".####...");
    }
}
