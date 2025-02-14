#![feature(never_patterns)]
#![allow(incomplete_features)]

enum Void {}

fn by_value(s: Void) {
    || { let ! = s; };
}

fn main() {}
