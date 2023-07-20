// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() { let _: &'a (); 3 }

fn issue_75323_and_74447_1<'static>() -> &'a () {
    test::<{ let _: &'static (); 3 },>();
   //~^ ERROR a non-static lifetime is not allowed in a `const`
    &()
}

fn issue_75323_and_74447_2() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &(); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a usize| (), 0).1];
    //~^ ERROR a non-static lifetime is not allowed in a `const`
}

fn main() {}
