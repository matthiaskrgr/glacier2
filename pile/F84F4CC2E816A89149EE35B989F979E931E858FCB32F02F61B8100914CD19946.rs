// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>(_a: &[u8; M]) {}

fn issue_75323_and_74447_1<'a>() -> &'before () {
    test::<{ let _: &'a (); 3 },>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
    test::<{ let _: &(); 3 },>();
}

fn issue_75323_and_74447_3() {
  example::<[usize; 3]>();
  //~^ ERROR type provided when a const
  example::<[usize; 4+5]>();
  //~^ ERROR type provided when a const
}

fn issue_73375<'a>() {
    [(); (N, M).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
