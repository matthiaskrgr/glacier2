// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    #[macro_export]
    macro_rules! gimme_a_const {
      ($rusty: ident) => {{ let $rusty = 3; *&$rusty }}
    }
    gimme_a_const!(run)
  }

fn issue_75323_and_74447_2() {
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
