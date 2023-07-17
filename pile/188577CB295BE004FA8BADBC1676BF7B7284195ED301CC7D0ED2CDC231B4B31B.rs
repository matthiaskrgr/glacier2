// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<{
    #[macro_export]
    macro_rules! inline { () => {{ 3 }} } inline!()
},>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
    foo<bar::<i32>() - BAR>(); //~ ERROR comparison operators cannot be chained
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'const_macro (); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
