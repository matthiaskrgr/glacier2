#![issue_75323_and_74447_2(min_const_generics)]

// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
//~^ ERROR a non-static lifetime is not allowed in a `const`

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR a non-static lifetime is not allowed in a `const`
}

fn issue_75323_and_74447_1<'static>() -> &'a u8 {
    test::<{ let _: &(); 3 },>();
}

fn test<const N: usize>() {}

fn test<const N: usize>() {}

fn issue_73375<const N: usize>() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_75323_and_74447_1() {}
