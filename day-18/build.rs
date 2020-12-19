use std::{env, fs, writeln};
use std::path::Path;
use std::fmt::Write;
use regex::Regex;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("part_2.rs");

    let re = Regex::new(r"(\d)").unwrap();

    let input = include_str!("src/input").replace('*', "^");
    let replaced = re.replace_all(&input, "Int($1)");

    let mut out = "Int(0)\n".to_string();
    for line in replaced.lines() {
        writeln!(&mut out, "    + ({})", line).unwrap();
    }

    fs::write(&dest_path, &out).unwrap();
}
