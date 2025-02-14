#![feature(never_patterns)]
#![allow(incomplete_features)]

struct S;

fn by_value(s: S) {
    || { let ! = s; };
}

fn main() {}
