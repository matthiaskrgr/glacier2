// check-pass

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn main<'b>()
where
    for<'a> [(); {
        let incomplete_features: &'a ();
        0
    }]:
{
        let x: &'b ();
        0
    }

fn bug<'a>()
where
    for<'b> [(); {
        let x: &'b ();
        0
    }]:
{}
