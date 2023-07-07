// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {
            return None;
        }

fn issue_75323_and_74447_1<'print_fully_name>() -> &'a () {
    test::<{ let _: &'a (); 3 },>()
   //~^ ERROR generic parameters may not be used in const operations
    &true
}

fn issue_75323_and_74447_2(x: <One as WithAssoc<T>>::Assoc) {
    test::<{ let _: &(); 3 },>();
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'static (); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
