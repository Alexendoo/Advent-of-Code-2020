#[derive(Debug)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

use Instruction::*;

#[derive(Debug)]
struct Op {
    instruction: Instruction,
    argument: i32,
    seen: bool,
}

impl Op {
    fn parse(line: &str) -> Self {
        let instruction = match &line[..3] {
            "acc" => Acc,
            "jmp" => Jmp,
            "nop" => Nop,
            _ => unreachable!(),
        };

        Self {
            instruction,
            argument: line[4..].parse().unwrap(),
            seen: false,
        }
    }
}

fn main() {
    let input = include_str!("input");
    let mut ops: Vec<Op> = input.lines().map(Op::parse).collect();

    let mut index = 0;
    let mut acc = 0;
    loop {
        let next = &mut ops[index as usize];
        if next.seen {
            break;
        }

        match next.instruction {
            Acc => {
                acc += next.argument;
                index += 1;
            }
            Jmp => index += next.argument,
            Nop => index += 1,
        };

        next.seen = true;
    }

    println!("Part 1: {}", acc);
}
