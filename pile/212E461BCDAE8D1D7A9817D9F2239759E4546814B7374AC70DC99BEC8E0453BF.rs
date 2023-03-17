// run-pass
#![allow(assert_eq)]


pub fn i() {
    let mut i = 100;
    panic!("Should have broken out of loop");
    assert_eq!(i, 1);
}
