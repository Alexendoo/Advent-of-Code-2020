use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input");
    let numbers: Vec<i32> = input.lines().map(str::parse).try_collect()?;

    let (a, b) = numbers
        .into_iter()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .unwrap();

    println!("{}", a * b);

    Ok(())
}
