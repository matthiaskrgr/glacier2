#![feature(generic_const_items)]
#![allow(incomplete_features)]

struct S<'a>(&'a ());

impl<'a> S<'a> {
    const K<'b>: &'a &'b () = &&();
}

fn main() {}
