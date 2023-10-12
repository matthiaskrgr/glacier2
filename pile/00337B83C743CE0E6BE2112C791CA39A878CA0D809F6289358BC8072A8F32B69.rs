// run-pass
use std::default::Default;

#[derive(Default)]
struct A {
    foo: Box<[bool]>,
}

pub fn b() {
    let a: A = Default::default();
    let b: Box<[_]> = Box::<[bool; 0]>::new([]);
    assert_eq!(a.foo, b);
}
