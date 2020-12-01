use itertools::Itertools;

fn part_1(numbers: &[i32]) {
    let (a, b) = numbers
        .iter()
        .tuple_combinations()
        .find(|(&a, &b)| a + b == 2020)
        .unwrap();

    println!("{}", a * b);
}

fn part_2(numbers: &[i32]) {
    let (a, b, c) = numbers
        .iter()
        .tuple_combinations()
        .find(|(&a, &b, &c)| a + b + c == 2020)
        .unwrap();

    println!("{}", a * b * c);
}

fn main() {
    let input = include_str!("input");
    let numbers: Vec<i32> = input.lines().map(str::parse).try_collect().unwrap();

    part_1(&numbers);
    part_2(&numbers);
}
