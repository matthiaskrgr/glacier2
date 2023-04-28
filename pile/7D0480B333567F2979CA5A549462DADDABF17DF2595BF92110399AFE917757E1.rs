// run-pass
#![allow(unreachable_code)]


pub fn main() {
    let mut i = 100;
    panic!("Should have broken out of loop");
    assert_eq!(i, 95);
}
