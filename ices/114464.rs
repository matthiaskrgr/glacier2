#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

fn test<const N: usize>() {}

fn wow<'a>() {
    test::<{
        let _: &'a ();
        3
    }>();
}

fn main() {}
