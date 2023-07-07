//~^ ERROR expected trait, found constant `BAR`
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'foo () {
    test::<{ let _: &'a (); 3 },>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
    foo::<BAR + BAR>();
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_data: [u32; N]| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
