fn main() {
    let input = include_str!("input");

    let sum: u32 = input
        .split("\n\n")
        .map(|group| {
            let bitset = group
                .bytes()
                .fold(0, |acc: u32, ch| acc | (1 << ch.saturating_sub(b'a' - 1)))
                >> 1;

            bitset.count_ones()
        })
        .sum();

    println!("Part 1: {}", sum);
}
