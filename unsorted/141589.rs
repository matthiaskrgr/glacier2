#![feature(never_patterns)]
#![allow(incomplete_features)]
enum Void {}

enum Either {
    One(X),
    Two(X),
}

struct X;

fn move_into_fnmut() {
    let x = Either::One(X);
    || {
        let Either::Two(!) = x;
    };
}

fn main() {}
