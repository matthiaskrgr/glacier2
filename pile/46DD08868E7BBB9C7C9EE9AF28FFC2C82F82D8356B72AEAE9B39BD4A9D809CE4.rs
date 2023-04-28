// Test that a variable of type ! can coerce to another type.

// check-pass

#![feature(never_type)]

fn panic() {
    let x: ! = panic!();
    let y: u32 = x;
}
