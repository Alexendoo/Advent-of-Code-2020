use fixedbitset::FixedBitSet;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

use Instruction::*;

type Op = (Instruction, i32);

fn parse(line: &str) -> Op {
    let instruction = match &line[..3] {
        "acc" => Acc,
        "jmp" => Jmp,
        "nop" => Nop,
        _ => unreachable!(),
    };

    (instruction, line[4..].parse().unwrap())
}

fn run(ops: &[Op]) -> (i32, bool, FixedBitSet) {
    let mut visited = FixedBitSet::with_capacity(ops.len());
    let mut cursor = 0;
    let mut acc = 0;

    let len = ops.len() as i32;

    while cursor < len && !visited.put(cursor as usize) {
        match ops[cursor as usize] {
            (Acc, i) => acc += i,
            (Jmp, i) => cursor += i - 1,
            (Nop, _) => {}
        }

        cursor += 1;
    }

    (acc, cursor == len, visited)
}

fn swap(ops: &mut [Op], index: usize) {
    let (op, _) = &mut ops[index];

    *op = match op {
        Acc => Acc,
        Jmp => Nop,
        Nop => Jmp,
    };
}

fn main() {
    let input = include_str!("input");
    let mut ops: Vec<Op> = input.lines().map(parse).collect();

    let (acc, _, visited) = run(&ops);
    println!("Part 1: {}", acc);

    for index in visited.ones() {
        swap(&mut ops, index);

        let (acc, complete, _) = run(&ops);
        if complete {
            println!("Part 2: {}", acc);
            break;
        }

        swap(&mut ops, index);
    }
}
