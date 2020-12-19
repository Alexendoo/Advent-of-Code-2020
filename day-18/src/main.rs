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

fn part_2() -> usize {
    struct Int(usize);

    impl std::ops::Add for Int {
        type Output = Int;

        fn add(self, other: Self) -> Self {
            Self(self.0 + other.0)
        }
    }

    impl std::ops::BitXor for Int {
        type Output = Self;

        fn bitxor(self, other: Self) -> Self {
            Self(self.0 * other.0)
        }
    }

    include!(concat!(env!("OUT_DIR"), "/part_2.rs")).0
}

fn main() {
    let input = include_str!("input");

    let sum: usize = input
        .lines()
        .map(|line| calc(&mut line.chars().filter(char::is_ascii_graphic)))
        .sum();

    println!("Part 1: {}", sum);
    println!("Part 2: {}", part_2());
}
