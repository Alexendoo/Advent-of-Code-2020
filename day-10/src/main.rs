use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input");
    let mut adapters: Vec<usize> = input.lines().map(str::parse).try_collect()?;
    adapters.push(0);
    adapters.sort_unstable();

    let (ones, threes) = adapters.iter().tuple_windows().map(|(a, b)| b - a).fold(
        (0, 0),
        |(ones, threes), difference| match difference {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => unreachable!(),
        },
    );

    println!("Part 1: {}", ones * (threes + 1));

    Ok(())
}
