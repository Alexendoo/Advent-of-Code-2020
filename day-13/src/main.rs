fn main() {
    let input = include_str!("input");
    let mut lines = input.lines();

    let start: u32 = lines.next().unwrap().parse().unwrap();
    let (wait, bus) = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .map(|bus: u32| (bus - (start % bus), bus))
        .min()
        .unwrap();

    println!("Part 1: {}", wait * bus);
}
