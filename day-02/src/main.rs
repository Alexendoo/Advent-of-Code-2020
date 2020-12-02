use regex::Regex;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    ch: char,
    password: &'static str,
}

fn parse(line: &'static str, pattern: &Regex) -> Policy {
    let captures = pattern.captures(line).unwrap();

    let min: usize = captures[1].parse().unwrap();
    let max: usize = captures[2].parse().unwrap();
    let ch = captures[3].chars().next().unwrap();
    let password = captures.get(4).unwrap().as_str();

    Policy {
        min,
        max,
        ch,
        password,
    }
}

fn part_1(policies: &[Policy]) {
    let valid = policies
        .iter()
        .filter(|&policy| {
            let range = policy.min..=policy.max;
            let matching_chars = policy
                .password
                .chars()
                .filter(|&ch| ch == policy.ch)
                .count();

            range.contains(&matching_chars)
        })
        .count();

    println!("{}", valid);
}

fn part_2(policies: &[Policy]) {
    let valid = policies
        .iter()
        .filter(|&policy| {
            let get_char = |i| policy.password.chars().nth(i - 1).unwrap();

            let first = get_char(policy.min);
            let second = get_char(policy.max);

            first != second && (first == policy.ch || second == policy.ch)
        })
        .count();

    println!("{}", valid);
}

fn main() {
    let input = include_str!("input");
    let pattern = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let policies: Vec<Policy> = input.lines().map(|line| parse(line, &pattern)).collect();

    part_1(&policies);
    part_2(&policies);
}
