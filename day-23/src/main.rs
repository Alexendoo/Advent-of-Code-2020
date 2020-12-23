use itertools::Itertools;

// cups[n] is the next cup after n

fn pick_up(cups: &[u32], current: u32) -> [u32; 3] {
    let p1 = cups[current as usize];
    let p2 = cups[p1 as usize];
    let p3 = cups[p2 as usize];

    [p1, p2, p3]
}

fn extract_cups(cups: &[u32]) -> Vec<u32> {
    let mut current = cups[0];
    let mut out = Vec::with_capacity(cups.len());

    for _ in 0..cups.len() - 1 {
        out.push(current);

        current = cups[current as usize];
    }

    out
}

fn play(mut cups: Vec<u32>, rounds: usize) -> Vec<u32> {
    for _ in 0..rounds {
        let current = cups[0];
        let picked = pick_up(&cups, current);

        let mut destination = current - 1;
        while picked.contains(&destination) {
            destination -= 1;
        }

        if destination == 0 {
            destination = cups.len() as u32 - 1;

            while picked.contains(&destination) {
                destination -= 1;
            }
        }

        cups[current as usize] = cups[picked[2] as usize];
        cups[picked[2] as usize] = cups[destination as usize];
        cups[destination as usize] = picked[0];
        cups[0] = cups[current as usize];
    }

    cups
}

fn main() {
    let input: Vec<u32> = "362981754".bytes().map(|b| b - b'0').map_into().collect();

    let mut cups = vec![0; input.len() + 1];

    let mut last = 0;
    for i in input {
        cups[last] = i;
        last = i as usize;
    }
    cups[last] = cups[0];

    let mut part_1 = play(cups.clone(), 100);
    part_1[0] = 1;

    println!(
        "Part 1: {}",
        extract_cups(&part_1).into_iter().skip(1).join("")
    );

    let total_cups = 1_000_000;

    let len = cups.len();
    cups.resize(total_cups + 1, 0);
    for i in len..=total_cups {
        cups[last] = i as u32;
        last = i as usize;
    }
    cups[last] = cups[0];

    let part_2 = play(cups, 10_000_000);

    let c1 = part_2[1] as u64;
    let c2 = part_2[c1 as usize] as u64;

    println!("Part 2: {}", c1 * c2);
}
