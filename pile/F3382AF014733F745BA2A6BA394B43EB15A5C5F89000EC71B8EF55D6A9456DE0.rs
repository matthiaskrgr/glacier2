#![feature(core, unboxed_closures)]
use std::ops::Fn;
fn say(x: u32, y: u32) {
    println!("{} {}", x, y);
}
fn main() {
    Fn::Fn(&say, (1, 2));
}
