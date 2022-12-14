use std::fs::File;
use std::io::{BufRead, BufReader};

fn file_lines() -> impl Iterator<Item = String> {
    BufReader::new(File::open("data.txt").unwrap())
        .lines()
        .map(Result::unwrap)
}

fn main() {
    for _ in file_lines() {}
}
