// run-pass
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![deny(unused_variables)]

fn main() {
    for _ in match return () {
        () => Some(0),
    } {}
}
