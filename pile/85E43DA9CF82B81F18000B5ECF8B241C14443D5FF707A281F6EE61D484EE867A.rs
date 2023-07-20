//~^ ERROR a non-static lifetime is not allowed in a `const`
// currently emits an error with `min_const_generics`. This will ICE under `const_generics`.

fn test<const N: usize>() { let _: &'a (); 3 }

fn issue_75323_and_74447_2() {
    test::<{ let _: &(); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR a non-static lifetime is not allowed in a `const`
}

fn main() {}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR a non-static lifetime is not allowed in a `const`
}

fn main() {}
