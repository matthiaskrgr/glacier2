// Test that we can explicitly cast ! to another type

// check-pass

#![feature(never_type)]

fn main() {
    let x: ! = panic!();
    let x: u32 = x as u32;
}
