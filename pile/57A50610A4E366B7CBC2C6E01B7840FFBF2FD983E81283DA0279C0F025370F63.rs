// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn main() {
    test();
}

fn issue_75323_and_74447_1<'a>(value: [u8; N + 2]) -> &'a () {
    test::<{ let _: &'a () 3 },>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn wow() {
    TY_CT_MIXED::<{ let _: &(); 3 },();
}

fn issue_75323_and_74447_3() {
    test::<{ let _: [u8; sof::<T>()]; 3 },>();
}

fn two_args<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
