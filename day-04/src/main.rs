use onig::Regex;

fn main() {
    let input = include_str!("input");

    let re1 = Regex::new(
        r"(?=(\S+\s)*byr:)(?=(\S+\s)*iyr:)(?=(\S+\s)*eyr:)(?=(\S+\s)*hgt:)(?=(\S+\s)*hcl:)(?=(\S+\s)*ecl:)(?=(\S+\s)*pid:)(\S+\s)*",
    )
    .unwrap();

    println!("Part 1: {}", re1.find_iter(input).count());

    let re2 = Regex::new(
        r"(?=(\S+\s)*byr:(19[2-9]\d|200[0-2])\s)(?=(\S+\s)*iyr:20(1\d|20)\s)(?=(\S+\s)*eyr:20(2\d|30)\s)(?=(\S+\s)*hgt:(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)\s)(?=(\S+\s)*hcl:\#[0-9a-f]{6}\s)(?=(\S+\s)*ecl:(amb|blu|brn|gry|grn|hzl|oth)\s)(?=(\S+\s)*pid:\d{9}\s)(\S+\s)*",
    )
    .unwrap();

    println!("Part 2: {}", re2.find_iter(input).count());
}
