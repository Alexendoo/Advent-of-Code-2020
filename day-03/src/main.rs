fn ascii_char_at(s: &str, index: usize) -> char {
    s.as_bytes()[index] as char
}

fn count_trees(rows: &[&str], right: usize, down: usize) -> usize {
    let width = rows[0].len();

    rows.iter()
        .step_by(down)
        .enumerate()
        .map(|(index, &row)| ascii_char_at(row, (index * right) % width))
        .filter(|&square| square == '#')
        .count()
}

fn main() {
    let input = include_str!("input");
    let rows: Vec<&str> = input.lines().collect();

    println!("Part 1: {}", count_trees(&rows, 3, 1));

    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: usize = slopes
        .iter()
        .map(|&(right, down)| count_trees(&rows, right, down))
        .product();

    println!("Part 2: {}", product);
}
