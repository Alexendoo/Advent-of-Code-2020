use std::iter::repeat;
use std::mem;

#[derive(Copy, Clone, PartialEq)]
enum Square {
    Empty,
    Occupied,
    Floor,
}
use Square::*;

impl Square {
    fn new(ch: u8) -> Self {
        match ch {
            b'L' => Empty,
            b'#' => Occupied,
            b'.' => Floor,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Empty => 'L',
            Occupied => '#',
            Floor => '.',
        };

        write!(f, "{}", ch)
    }
}

fn tick(grid: &Vec<Square>, width: usize, height: usize) -> Vec<Square> {
    let mut new_grid = grid.clone();

    let at = |x, y| x + y * width;
    let sq = |x, y| grid[at(x, y)];
    #[rustfmt::skip]
    let surroundings = |x, y| [
        sq(x - 1, y - 1), sq(x, y - 1), sq(x + 1, y - 1),
        sq(x - 1, y),                   sq(x + 1, y),
        sq(x - 1, y + 1), sq(x, y + 1), sq(x + 1, y + 1),
    ];

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let index = at(x, y);
            let occupied = surroundings(x, y)
                .iter()
                .filter(|&&s| s == Occupied)
                .count();

            new_grid[index] = match grid[index] {
                Empty if occupied == 0 => Occupied,
                Occupied if occupied >= 4 => Empty,
                other => other,
            };
        }
    }

    new_grid
}

fn print_grid(grid: &[Square], width: usize) {
    for g in grid[width..(grid.len() - width)].chunks(width) {
        for s in &g[1..(width - 1)] {
            print!("{:?}", s);
        }
        println!();
    }
}

fn main() {
    let input = include_str!("input");

    let width = input.find("\n").unwrap() + 2;

    let mut grid = vec![Floor; width];
    for line in input.lines() {
        grid.push(Floor);
        for ch in line.bytes() {
            grid.push(Square::new(ch))
        }
        grid.push(Floor);
    }
    grid.extend(repeat(Floor).take(width));

    let height = grid.len() / width;

    loop {
        let new_grid = tick(&grid, width, height);
        let old_grid = mem::replace(&mut grid, new_grid);

        if grid == old_grid {
            break;
        }
    }

    let occupied = grid.iter().filter(|&&sq| sq == Occupied).count();

    println!("Part 1: {}", occupied);
}
