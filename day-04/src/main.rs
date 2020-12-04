use onig::Regex;

fn main() {
    let input = include_str!("input");

    let re1 = Regex::new(
        r"(?=(\S+\s)*byr:)(?=(\S+\s)*iyr:)(?=(\S+\s)*eyr:)(?=(\S+\s)*hgt:)(?=(\S+\s)*hcl:)(?=(\S+\s)*ecl:)(?=(\S+\s)*pid:)(\S+\s)*",
    )
    .unwrap();

    println!("Part 1: {}", re1.find_iter(input).count());
}
