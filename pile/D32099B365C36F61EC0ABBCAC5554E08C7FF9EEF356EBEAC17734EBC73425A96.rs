// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    foo::<3 + 3>(); //~ ERROR expressions must be enclosed in braces
}

fn issue_75323_and_74447_2(_data: [u32; N]) {
    test::<{ let _: &'a (); 3 },>();
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {
    foo<bar<i32>()>(); //~ ERROR comparison operators cannot be chained
    //~^ ERROR expected one of `;` or `}`, found `>`
}
