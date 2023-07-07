// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const SIZE1: usize, const SIZE2: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'b () {
    test::<generic_arg>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
    test::<bar(1, 1)>();
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'static (); 3 },>()
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
