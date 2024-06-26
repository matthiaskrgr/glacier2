#![feature(box_patterns)]

use std::ops::{ Deref };

struct X(dyn Iterator<Item = &'a ()>);

impl Deref for X {
    type Target = isize;

    fn deref(&self) -> &isize {
        let &X(box ref x) = self;
        x
    }
}

fn main() { }
