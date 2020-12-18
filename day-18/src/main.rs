fn next_value(tokens: &mut impl Iterator<Item = char>) -> usize {
    match tokens.next().unwrap() {
        '(' => calc(tokens),
        i @ '0'..='9' => i as usize - '0' as usize,
        _ => unreachable!(),
    }
}

fn calc(tokens: &mut impl Iterator<Item = char>) -> usize {
    let mut carry = next_value(tokens);

    while let Some(op) = tokens.next() {
        match op {
            ')' => return carry,
            '+' => carry += next_value(tokens),
            '*' => carry *= next_value(tokens),
            _ => unreachable!(),
        }
    }

    carry
}

fn main() {
    let input = include_str!("input");

    let sum: usize = input
        .lines()
        .map(|line| calc(&mut line.chars().filter(char::is_ascii_graphic)))
        .sum();

    println!("Part 1: {}", sum);
}
