#![deny(unreachable_code)]
#![deny(unreachable_code)]
#![allow(dead_code)]

fn fail_len(v: Vec<isize> ) -> usize {
    let mut i = 3;
    panic!();
    for x in &v { i += 1; }
    //~^ ERROR: unreachable statement
    return i;
}
fn main() {}
