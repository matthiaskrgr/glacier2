// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<{ let _: &'a (); 3 },>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn b() {
    foo<BAR + BAR>(); //~ ERROR comparison operators cannot be chained
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'_fail (); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a char| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
