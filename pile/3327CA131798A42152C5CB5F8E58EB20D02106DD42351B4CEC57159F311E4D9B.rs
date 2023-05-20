#![feature(const_generics)]
#![allow(incomplete_features)]

fn test<const N: usize>() {}

fn wow<'a>() {
    test::<{
    test::<{
        let _: &'a ();
        3
    }>();
}>();
}

fn main() {}
