use std::cmp::Ordering;
use std::collections::HashSet;

fn bag_name(input: &str) -> &str {
    let end = input.find(" bag").unwrap();

    &input[..end]
}

type Edges = Vec<(&'static str, &'static str)>;

fn find_containers_shallow<'a>(bag: &'a str, edges: &'a Edges) -> impl Iterator<Item = &'a str> {
    let start = edges
        .binary_search_by(|&(_, containee)| containee.cmp(bag).then(Ordering::Greater))
        .unwrap_err();

    edges
        .iter()
        .copied()
        .skip(start)
        .take_while(move |&(_, containee)| containee == bag)
        .map(|(container, _)| container)
}

fn find_containers_deep<'a>(bag: &'a str, edges: &'a Edges) -> HashSet<&'a str> {
    let mut queue = vec![bag];
    let mut found = HashSet::new();

    while let Some(next) = queue.pop() {
        let start_len = queue.len();

        queue.extend(find_containers_shallow(next, edges));
        found.extend(&queue[start_len..]);
    }

    found
}

fn main() {
    let input = include_str!("input");

    let mut edges = Vec::new();

    for line in input.lines() {
        let container = bag_name(line);

        let contents = line.trim_start_matches(|ch: char| !ch.is_ascii_digit());
        if contents.is_empty() {
            continue;
        }

        for content in contents.split(", ") {
            let mut split = content.splitn(2, ' ');
            let _count = split.next().unwrap();
            let bag = bag_name(split.next().unwrap());

            edges.push((container, bag));
        }
    }

    edges.sort_unstable_by_key(|&(_, containee)| containee);

    let containers = find_containers_deep("shiny gold", &edges);

    println!("Part 1: {}", containers.len());
}
