// Test that an assignment of type ! makes the rest of the block dead code.

//~^ WARN unreachable

#![feature(never_type)]
#![warn(unused)]

fn main() {
    let x: ! = panic!("aah"); //~ WARN unused
    drop(x); //~ WARN unreachable
    //~^ WARN unreachable
}
