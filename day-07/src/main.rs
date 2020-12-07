use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
struct Edge {
    parent: &'static str,
    child: &'static str,
    count: usize,
}

fn find_edges<'a, F>(edges: &'a [Edge], bag: &'a str, f: F) -> impl Iterator<Item = Edge> + 'a
where
    F: Fn(&Edge) -> &str + 'a,
{
    let start = edges
        .binary_search_by(|edge| f(edge).cmp(bag).then(Ordering::Greater))
        .unwrap_err();

    edges[start..]
        .iter()
        .take_while(move |edge| f(edge) == bag)
        .copied()
}

fn parents<'a>(edges: &'a [Edge], bag: &'a str) -> HashSet<&'a str> {
    let mut queue = vec![bag];
    let mut found = HashSet::new();

    while let Some(next) = queue.pop() {
        for edge in find_edges(edges, next, |edge| edge.child) {
            queue.push(edge.parent);
            found.insert(edge.parent);
        }
    }

    found
}

fn children(edges: &[Edge], bag: &str) -> usize {
    find_edges(edges, bag, |edge| edge.parent)
        .map(|edge| edge.count + edge.count * children(edges, edge.child))
        .sum()
}

fn bag_name(input: &str) -> &str {
    let end = input.find(" bag").unwrap();

    &input[..end]
}

fn parse_edges() -> Vec<Edge> {
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

    edges
}

fn main() {
    let mut edges = parse_edges();

    edges.sort_unstable_by_key(|edge| edge.child);
    println!("Part 1: {}", parents(&edges, "shiny gold").len());

    edges.sort_unstable_by_key(|edge| edge.parent);
    println!("Part 2: {}", children(&edges, "shiny gold"));
}
