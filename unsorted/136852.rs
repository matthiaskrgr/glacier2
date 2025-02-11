#![feature(never_patterns)]
#![allow(incomplete_features)]

fn by_value(s: usize) {
    || { let ! = s; };
}

fn main() {}
