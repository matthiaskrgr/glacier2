// Test that a variable of type ! can coerce to another type.

// Test that a variable of type ! can coerce to another type.

#![feature(never_type)]

fn main() {
    let x: ! = panic!();
    let y: u32 = x;
}