// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<{ let _: &'a (); 3 },>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
    test::<{ let _: &(); 3 },>(size_of::<*mut T>());
}

fn issue_75323_and_74447_3() {
    test::<{
    foo<BAR - 3>(); //~ ERROR comparison operators cannot be chained
},>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
