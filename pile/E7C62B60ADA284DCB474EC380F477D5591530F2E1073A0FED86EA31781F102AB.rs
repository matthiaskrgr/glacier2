#![feature(const_generics)]
#![feature(const_generics)]

fn test<const N: usize>() {}

fn wow<'a>() {
    test::<{
        let _: &'a ();
        3
    }>();
}

fn main() {}
