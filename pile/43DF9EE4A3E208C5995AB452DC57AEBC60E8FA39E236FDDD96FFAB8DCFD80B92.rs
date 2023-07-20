// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It

fn test<const static: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'static () {
    [(); (|_: &'a u8| (), 0).1];
   // currently emits an error with `min_const_generics`.
    &()
}

fn static() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_75323_and_74447_3() {
    [(); (|_: &'a u8| (), 0).1];
}

fn issue_75323_and_74447_2<'a>() {
    [(); (|_: &'a usize| (), 0).1];
    //~^ ERROR a non-static lifetime is not allowed in a `const`
}

fn main() {}
