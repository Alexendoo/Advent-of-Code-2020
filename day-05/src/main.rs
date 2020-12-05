use std::mem::size_of;
use std::slice;

#[repr(C)]
struct Pass {
    row_bytes: [u8; 7],
    col_bytes: [u8; 3],

    _newline: u8,
}

fn eq_b(ch: u8) -> u8 {
    (ch == b'B') as u8
}

fn eq_r(ch: u8) -> u8 {
    (ch == b'R') as u8
}

impl Pass {
    fn row(&self) -> u8 {
        let r = self.row_bytes;

        eq_b(r[0]) << 6
            | eq_b(r[1]) << 5
            | eq_b(r[2]) << 4
            | eq_b(r[3]) << 3
            | eq_b(r[4]) << 2
            | eq_b(r[5]) << 1
            | eq_b(r[6])
    }

    fn col(&self) -> u8 {
        let c = self.col_bytes;

        eq_r(c[0]) << 2 | eq_r(c[1]) << 1 | eq_r(c[2])
    }

    fn id(&self) -> u32 {
        (self.row() as u32) * 8 + (self.col() as u32)
    }
}

fn sum_between(start: u32, end: u32) -> u32 {
    (end - start + 1) * (start + end) / 2
}

fn main() {
    let input = include_bytes!("input");
    let passes = unsafe {
        let ptr = input as *const u8 as *const Pass;
        let len = input.len() / size_of::<Pass>();
        slice::from_raw_parts(ptr, len)
    };

    let mut min_id = u32::MAX;
    let mut max_id = 0;
    let mut sum_ids = 0;

    for pass in passes {
        let id = pass.id();

        min_id = min_id.min(id);
        max_id = max_id.max(id);
        sum_ids += id;
    }

    println!("Part 1: {}", max_id);
    println!("Part 2: {}", sum_between(min_id, max_id) - sum_ids);
}
