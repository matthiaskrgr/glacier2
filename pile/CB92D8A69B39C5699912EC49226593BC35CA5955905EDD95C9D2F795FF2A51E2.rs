// run-pass
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unreachable_code)]

fn main() {
    for _ in match return () {
        () => Some(0),
    } {
    for _ in match return () {
        () => Some(0),
    } {}
}
}
