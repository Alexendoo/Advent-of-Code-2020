fn bitset(answers: &str) -> u32 {
    answers
        .bytes()
        .fold(0, |acc, ch| acc | (1 << ch.saturating_sub(b'a' - 1)))
        >> 1
}

fn main() {
    let input = include_str!("input");

    let sum1: u32 = input
        .split("\n\n")
        .map(|group| bitset(group).count_ones())
        .sum();

    println!("Part 1: {}", sum1);

    let sum2: u32 = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(!0, |acc, line| acc & bitset(line))
                .count_ones()
        })
        .sum();

    println!("Part 2: {}", sum2);
}
