use itertools::Itertools;
use onig::{Captures, Regex};

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).unwrap()
}

fn count_matches(rules: &str, messages: &str) -> usize {
    let without_quotes = rules.replace('"', "");
    let rule_groups = re(r"(.*): (.*)").replace_all(&without_quotes, |caps: &Captures| {
        format!("(?<c{}> {})", caps.at(1).unwrap(), caps.at(2).unwrap())
    });
    let subcalls = re(r" (\d+)").replace_all(&rule_groups, |caps: &Captures| {
        format!(r" \g<c{}>", caps.at(1).unwrap())
    });
    let pattern = format!(r"(?x) ^\g<c0>$ (?:{}){{0}}", subcalls);

    re(&pattern).find_iter(messages).count()
}

fn main() {
    let input = include_str!("input");
    let (rules, messages) = input.split("\n\n").collect_tuple().unwrap();

    println!("Part 1: {}", count_matches(rules, messages));

    let updated_rules = rules
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");

    println!("Part 2: {}", count_matches(&updated_rules, messages));
}
