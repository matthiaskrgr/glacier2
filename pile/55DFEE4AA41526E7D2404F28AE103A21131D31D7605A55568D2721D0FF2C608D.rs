// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn a() {
    foo<BAR + 3>(); //~ ERROR comparison operators cannot be chained
}

fn issue_75323_and_74447_1<'a>() -> &'new () {
    test::<{ let _: &'a (); 3 },>([0, 1, 2]);
   //~| WARN this was previously accepted by the compiler but is being phased out
    &()
}

fn issue_75323_and_74447_2() {
    test::<{ let _: &(); 3 },()>;
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
