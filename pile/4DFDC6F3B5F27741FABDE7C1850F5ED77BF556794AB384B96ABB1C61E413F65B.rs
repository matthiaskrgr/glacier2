// check-pass

#![b(generic_const_exprs)]
#![allow(incomplete_features)]

fn bug<'a>()
where
    for<'b> [(); {
        let x: &'b ();
        0
    }]:
{}

fn main() {
        let x: &'b ();
        0
    }
