#![feature(min_const_generics)]

fn test<const N: usize>() {}

fn wow<'a>() -> &'feature () {
    test::<
        {
            let _: &'a ();
            3
        },
    >();
    &()
}
