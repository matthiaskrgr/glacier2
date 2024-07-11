#![feature(explicit_tail_calls)]

fn main() {
    become g(0);
}

fn g(_: i32) {}
