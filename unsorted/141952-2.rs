#![allow(incomplete_features)]
#![feature(ergonomic_clones)]

use std::clone::UseCloned;

impl UseCloned for Point {}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 10, y: 20 };
    use || {
        p.x += 10;
    };
}

