// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<{ let _: &'a (); 3 },>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn break_ty3<T>() {
    let _ = [0; size_of::<*mut T>() + 1];
    //~^ WARN cannot use constants which depend on generic parameters in types
    //~| WARN this was previously accepted by the compiler but is being phased out
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
