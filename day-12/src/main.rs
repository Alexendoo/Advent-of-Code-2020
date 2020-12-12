fn instructions() -> impl Iterator<Item = (char, i32)> {
    let input = include_str!("input");

    input.lines().map(|line| {
        let action = line.as_bytes()[0] as char;
        let value: i32 = line[1..].parse().unwrap();

        (action, value)
    })
}

fn part_1() {
    let mut direction = 90;
    let mut north = 0;
    let mut east = 0;

    for (action, value) in instructions() {
        match (action, direction) {
            ('N', _) => north += value,
            ('S', _) => north -= value,
            ('E', _) => east += value,
            ('W', _) => east -= value,

            ('R', _) => direction += value,
            ('L', _) => direction -= value,

            ('F', 0) => north += value,
            ('F', 90) => east += value,
            ('F', 180) => north -= value,
            ('F', 270) => east -= value,

            _ => unreachable!(),
        }

        direction = direction.rem_euclid(360);
    }

    println!("Part 1: {}", north.abs() + east.abs());
}

fn rotate_right(degrees: i32, north: &mut i32, east: &mut i32) {
    let n = *north;
    let e = *east;

    let rotated = match degrees.rem_euclid(360) {
        0 => (n, e),
        90 => (-e, n),
        180 => (-n, -e),
        270 => (e, -n),
        _ => unreachable!(),
    };

    *north = rotated.0;
    *east = rotated.1;
}

fn part_2() {
    let mut waypoint_north = 1;
    let mut waypoint_east = 10;

    let mut ship_north = 0;
    let mut ship_east = 0;

    for (action, value) in instructions() {
        match action {
            'N' => waypoint_north += value,
            'S' => waypoint_north -= value,
            'E' => waypoint_east += value,
            'W' => waypoint_east -= value,

            'L' => rotate_right(-value, &mut waypoint_north, &mut waypoint_east),
            'R' => rotate_right(value, &mut waypoint_north, &mut waypoint_east),

            'F' => {
                ship_north += value * waypoint_north;
                ship_east += value * waypoint_east;
            }

            _ => unreachable!(),
        }

    }

    println!("Part 2: {}", ship_north.abs() + ship_east.abs());
}

fn main() {
    part_1();
    part_2();
}
