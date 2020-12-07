use std::cmp::Ordering;
use std::collections::HashSet;

fn bag_name(input: &str) -> &str {
    let end = input.find(" bag").unwrap();

    &input[..end]
}

#[derive(Copy, Clone, Debug)]
struct Edge {
    parent: &'static str,
    child: &'static str,
    count: usize,
}

fn direct_parents<'a>(bag: &'a str, edges: &'a [Edge]) -> impl Iterator<Item = &'a str> {
    let start = edges
        .binary_search_by(|&edge| edge.child.cmp(bag).then(Ordering::Greater))
        .unwrap_err();

    edges
        .iter()
        .copied()
        .skip(start)
        .take_while(move |&edge| edge.child == bag)
        .map(|edge| edge.parent)
}

fn all_parents<'a>(bag: &'a str, edges: &'a [Edge]) -> HashSet<&'a str> {
    let mut queue = vec![bag];
    let mut found = HashSet::new();

    while let Some(next) = queue.pop() {
        let start_len = queue.len();

        queue.extend(direct_parents(next, edges));
        found.extend(&queue[start_len..]);
    }

    found
}

fn main() {
    let input = include_str!("input");

    let mut edges = Vec::new();

    for line in input.lines() {
        let parent = bag_name(line);

        let contents = line.trim_start_matches(|ch: char| !ch.is_ascii_digit());
        if contents.is_empty() {
            continue;
        }

        for content in contents.split(", ") {
            let mut split = content.splitn(2, ' ');
            let count = split.next().unwrap().parse().unwrap();
            let child = bag_name(split.next().unwrap());

            edges.push(Edge {
                parent,
                child,
                count,
            });
        }
    }

    edges.sort_unstable_by_key(|&edge| edge.child);
    eprintln!("edges = {:#?}", edges);

    let containers = all_parents("shiny gold", &edges);

    println!("Part 1: {}", containers.len());
}
