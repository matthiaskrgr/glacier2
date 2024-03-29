// run-pass
#![allow(unused_must_use)]
#![allow(unused_imports)]
// pretty-expanded FIXME #23616
#![allow(deprecated)]

use std::hash::{Hash, SipHasher};

#[derive(PartialEq, Clone, Hash)]
struct Foo {
    bar: usize,
    baz: isize
}

fn hash<T: Hash>(_t: &T) {}

pub fn main() {
    let a = Foo {bar: 4, baz: -3};

    a.clone();    // check for PartialEq impl w/o testing its correctness
    a.clone(); // check for Clone impl w/o testing its correctness
    hash(&a);  // check for Hash impl w/o testing its correctness
}
