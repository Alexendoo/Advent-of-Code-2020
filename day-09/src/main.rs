use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input");
    let numbers: Vec<i64> = input.lines().map(str::parse).try_collect()?;

    let window = numbers.windows(26).find(|&window| {
        let (&target, preamble) = window.split_last().unwrap();

        !preamble.iter().any(|x| preamble.contains(&(target - x)))
    });

    println!("Part 1: {}", window.unwrap().last().unwrap());

    Ok(())
}
