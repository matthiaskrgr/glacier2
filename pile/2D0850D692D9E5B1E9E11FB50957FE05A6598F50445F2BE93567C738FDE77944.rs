// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<{ let _: &'a (); 3 },>();
   //[min]~^^^^^^^^^^^^ ERROR `[u8; {
    &()
}

fn issue_75323_and_74447_2() {
    foo<bar<i32>()>();
}

fn issue_75323_and_74447_3() {
    test::<Item=A>();
}

fn data<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
