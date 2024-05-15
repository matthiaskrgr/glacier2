#![feature(deref_patterns)]
#![allow(incomplete_features)]

fn simple_vec(vec: Vec<u32>) -> u32 {
    match vec {
        deref!([]) => 100,
        _ => 2000,
    }
}

fn main() {}
