// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR a non-static lifetime is not allowed in a `const`
}

fn test<const N: usize>() {}

fn issue_75323_and_74447_3() { let _: &(); 3 }

fn issue_75323_and_74447_2<'a>() {}

fn main() {}
