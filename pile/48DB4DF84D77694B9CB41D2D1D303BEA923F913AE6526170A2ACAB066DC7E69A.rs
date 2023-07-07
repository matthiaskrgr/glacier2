// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test_2<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    std::mem::transmute(v)
    //~^ ERROR cannot transmute between types
  }

fn issue_75323_and_74447_2(x: T) {
    test::<{ let _: &(); 3 },>();
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'U (); 3 },>();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
