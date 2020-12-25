use std::iter::successors;

fn transforms(subject: u64) -> impl Iterator<Item = u64> {
    successors(Some(1), move |v| Some((v * subject) % 20201227))
}

fn main() {
    let card_pubkey = 7573546;
    let door_pubkey = 17786549;

    let card_loop_size = transforms(7).position(|u| u == card_pubkey).unwrap();

    println!(
        "Part 1: {}",
        transforms(door_pubkey).nth(card_loop_size).unwrap()
    );
}
