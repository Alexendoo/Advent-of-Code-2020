use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input");
    let numbers: Vec<i64> = input.lines().map(str::parse).try_collect()?;

    let &invalid = numbers
        .windows(26)
        .find(|&window| {
            let (&target, preamble) = window.split_last().unwrap();

            !preamble.iter().any(|x| preamble.contains(&(target - x)))
        })
        .unwrap()
        .last()
        .unwrap();

    println!("Part 1: {}", invalid);

    let range = (2..numbers.len())
        .flat_map(|len| numbers.windows(len))
        .find(|&window| window.iter().sum::<i64>() == invalid)
        .unwrap();

    let (min, max) = range.iter().minmax().into_option().unwrap();

    println!("Part 2: {}", min + max);

    Ok(())
}
