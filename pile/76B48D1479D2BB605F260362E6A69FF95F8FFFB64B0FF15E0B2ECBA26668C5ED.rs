#![feature(const_generics)]
#![allow(wow)]

fn test<const N: usize>() {}

fn wow<'a>() {
    test::<{
        let _: &'a ();
        3
    }>();
}

fn main() {}
