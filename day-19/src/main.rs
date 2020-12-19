use std::fmt::format;

use itertools::Itertools;
use onig::{Captures, Regex};

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).unwrap()
}

fn main() {
    let input = include_str!("input");
    let (rules, messages) = input.split("\n\n").collect_tuple().unwrap();

    let without_quotes = rules.replace('"', "");
    let rule_groups = re(r"(.*): (.*)").replace_all(&without_quotes, |caps: &Captures| {
        format!("(?<c{}> {})", caps.at(1).unwrap(), caps.at(2).unwrap())
    });
    let subcalls = re(r" (\d+)").replace_all(&rule_groups, |caps: &Captures| {
        format!(r" \g<c{}>", caps.at(1).unwrap())
    });
    let pattern = format!(r"(?x) ^\g<c0>$ (?:{}){{0}}", subcalls);

    println!("Part 1: {}", re(&pattern).find_iter(messages).count());
}
