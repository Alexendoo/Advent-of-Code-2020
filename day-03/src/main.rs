fn ascii_char_at(s: &str, index: usize) -> char {
    s.as_bytes()[index] as char
}

fn main() {
    let input = include_str!("input");
    let rows: Vec<&str> = input.lines().collect();
    let width = rows[0].len();

    let trees = rows
        .iter()
        .enumerate()
        .map(|(index, &row)| ascii_char_at(row, (index * 3) % width))
        .filter(|&square| square == '#')
        .count();

    println!("{}", trees);
}
