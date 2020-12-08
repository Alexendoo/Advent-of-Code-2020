use fixedbitset::FixedBitSet;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

use Instruction::*;

#[derive(Debug, Copy, Clone)]
struct Op {
    instruction: Instruction,
    argument: i32,
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
        }
    }
}

#[derive(Clone)]
struct State<'a> {
    ops: &'a [Op],
    cursor: i32,
    acc: i32,
    seen: FixedBitSet,
}

impl<'a> State<'a> {
    fn new(ops: &'a [Op]) -> Self {
        Self {
            ops: ops,
            cursor: 0,
            acc: 0,
            seen: FixedBitSet::with_capacity(ops.len()),
        }
    }

    fn next_op(&self) -> Op {
        self.ops[self.cursor as usize]
    }

    fn execute(&mut self, op: Op) -> bool {
        let seen = self.seen.put(self.cursor as usize);

        self.cursor += match op.instruction {
            Acc => {
                self.acc += op.argument;

                1
            }
            Jmp => op.argument,
            Nop => 1,
        };

        seen
    }

    fn advance(&mut self) -> bool {
        let next = self.next_op();

        self.execute(next)
    }

    fn run_to_end(&mut self) -> bool {
        loop {
            let len = self.ops.len() as i32;

            if self.cursor == len {
                return true;
            }

            if self.cursor > len || self.advance() {
                return false;
            }
        }
    }
}

fn main() {
    let input = include_str!("input");
    let ops: Vec<Op> = input.lines().map(Op::parse).collect();

    let mut state = State::new(&ops);

    state.run_to_end();

    println!("Part 1: {}", state.acc);

    let mut state = State::new(&ops);
    loop {
        let mut next = state.next_op();

        next.instruction = match next.instruction {
            Jmp => Nop,
            Nop => Jmp,

            Acc => {
                state.advance();
                continue;
            }
        };

        let mut test_state = state.clone();
        test_state.execute(next);
        if test_state.run_to_end() {
            println!("Part 2: {}", test_state.acc);
            break;
        }
        state.advance();
    }
}
