use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input");
    let mut adapters: Vec<usize> = input.lines().map(str::parse).try_collect()?;
    adapters.push(0);
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let differences = adapters
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    let (ones, threes) = differences
        .iter()
        .fold((0, 0), |(ones, threes), difference| match difference {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => unreachable!(),
        });

    println!("Part 1: {}", ones * threes);

    let product: usize = differences
        .split(|&diff| diff == 3)
        .map(|group| match group.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
        .product();

    println!("Part 2: {}", product);

    Ok(())
}
