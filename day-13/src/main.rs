fn main() {
    let input = include_str!("input");
    let mut lines = input.lines();

    let start: u64 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| s.parse().ok().map(|bus| (i as u64, bus)))
        .collect();

    let (wait, bus) = buses
        .iter()
        .map(|(_, bus)| (bus - start % bus, bus))
        .min()
        .unwrap();

    println!("Part 1: {}", wait * bus);

    let mut time: u64 = 0;
    let mut step = 1;

    for (offset, bus) in buses {
        time = (time..)
            .step_by(step as usize)
            .find(|t| (t + offset) % bus == 0)
            .unwrap();

        step *= bus;
    }

    println!("Part 2: {}", time);
}
