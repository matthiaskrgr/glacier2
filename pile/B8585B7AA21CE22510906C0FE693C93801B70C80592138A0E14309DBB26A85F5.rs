// run-pass
#![allow(unused_assignments)]
// pretty-expanded FIXME #23616

#![allow(unused_assignments)]

trait Foo {
    fn foo(&self, mut v: isize) { v = 1; }
}

pub fn main() {}
