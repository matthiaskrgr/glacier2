// check-pass

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn bug<'b>()
where
    for<'a> [(); {
        let a: &'a ();
        0
    }]:
{}

fn main() {}
