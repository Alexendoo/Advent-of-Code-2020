use regex::Regex;

fn part_1(input: &'static str) {
    let pattern = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let mut valid = 0;

    for line in input.lines() {
        let captures = pattern.captures(line).unwrap();

        let min: usize = captures[1].parse().unwrap();
        let max: usize = captures[2].parse().unwrap();
        let policy_char = captures[3].chars().next().unwrap();
        let password = &captures[4];

        let count = password.chars().filter(|&ch| ch == policy_char).count();

        if (min..=max).contains(&count) {
            valid += 1;
        }
    }

    println!("{}", valid);
}

fn main() {
    let input = include_str!("input");

    part_1(input);
}
