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
        Self {
            id: self.id,
            board: rotate_board(&self.board),
        }
    }

    fn flipped(&self) -> Self {
        Self {
            id: self.id,
            board: flip_board(&self.board),
        }
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

fn board_rows(board: &str, width: usize) -> Vec<&str> {
    (0..(8 * width))
        .map(|i| {
            let start = i * width * 8;
            let end = start + width * 8;

            &board[start..end]
        })
        .collect()
}

fn rotate_board(board: &str) -> String {
    let mut out = String::with_capacity(board.len());
    let width = sqrt(board.len());

    // # . .
    // . # .
    // . # .

    // . . .
    // . # #
    // # . .

    for col in (0..width).rev() {
        for row in 0..width {
            let i = row * width + col;
            let ch = board[i..].chars().next().unwrap();
            out.push(ch);
        }
    }

    assert_eq!(board.len(), out.len());

    out
}

fn flip_board(board: &str) -> String {
    let mut out = String::with_capacity(board.len());
    let width = sqrt(board.len());

    for row in (0..width).rev() {
        out.extend(board.chars().rev().skip(row * width).take(width));
    }

    assert_eq!(out.len(), board.len());

    out
}

fn find_monsters(board: &str, width: usize) -> Option<String> {
    let rows = board_rows(&board, width);

    // 01234567890123456789
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    let monster = &[
        &[18][..],
        &[0, 5, 6, 11, 12, 17, 18, 19],
        &[1, 4, 7, 10, 13, 16],
    ];

    let mut out = board.as_bytes().to_vec();
    let mut found = false;

    for (start_row, window) in rows.windows(3).enumerate() {
        for offset in 0..width * 8 - 20 {
            let has_monster = monster.iter().enumerate().all(|(row, cols)| {
                cols.iter()
                    .all(|col| window[row][offset + col..].starts_with('#'))
            });

            if has_monster {
                found = true;

                for (row, &cols) in monster.iter().enumerate() {
                    for col in cols {
                        let i = offset + col + (row + start_row) * width * 8;

                        out[i] = b'O';
                    }
                }
            }
        }
    }

    if found {
        Some(String::from_utf8(out).unwrap())
    } else {
        None
    }
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

    let mut board = String::new();
    for grid_row in 0..width {
        for board_row in 0..8 {
            for grid_col in 0..width {
                let grid_index = grid_row * width + grid_col;
                board.push_str(&grid[grid_index].row(board_row))
            }
        }
    }

    for _ in 0..4 {
        if let Some(board) =
            find_monsters(&board, width).or_else(|| find_monsters(&flip_board(&board), width))
        {
            let sea_tiles = board.chars().filter(|&ch| ch == '#').count();

            println!("Part 2: {}", sea_tiles);
        }

        board = rotate_board(&board);
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
