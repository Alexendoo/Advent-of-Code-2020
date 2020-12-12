fn main() {
    let input = include_str!("input");

    let mut direction = 90;
    let mut north = 0;
    let mut east = 0;
    for line in input.lines() {
        let action = line.as_bytes()[0];
        let value: i32 = line[1..].parse().unwrap();

        match (action, direction) {
            (b'N', _) => north += value,
            (b'S', _) => north -= value,
            (b'E', _) => east += value,
            (b'W', _) => east -= value,

            (b'R', _) => direction += value,
            (b'L', _) => direction -= value,

            (b'F', 0) => north += value,
            (b'F', 90) => east += value,
            (b'F', 180) => north -= value,
            (b'F', 270) => east -= value,

            _ => unreachable!(),
        }

        direction = direction.rem_euclid(360);
    }

    println!("Part 1: {}", north.abs() + east.abs());
}
