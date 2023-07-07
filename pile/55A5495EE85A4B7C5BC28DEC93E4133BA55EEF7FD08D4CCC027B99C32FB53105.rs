// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn main() {
  // Test that basic cases don't work
  assert!(get_flag::<true, 'c'>().is_some());
  assert!(get_flag::<false, 'x'>().is_none());
  get_flag::<false, 0xFF>();
  //~^ ERROR mismatched types
  get_flag::<7, 'c'>();
  //~^ ERROR mismatched types
  get_flag::<42, 0x5ad>();
  //~^ ERROR mismatched types
  //~| ERROR mismatched types


  get_flag::<false, { unsafe { char_raw.character } }>();
  //~^ ERROR evaluation of constant value failed
  //~| uninitialized
  get_flag::<{ unsafe { bool_raw.boolean } }, 'z'>();
  //~^ ERROR it is undefined behavior
  get_flag::<{ unsafe { bool_raw.boolean } }, { unsafe { char_raw.character } }>();
  //~^ ERROR evaluation of constant value failed
  //~| uninitialized
  //~| ERROR it is undefined behavior
}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<N>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
}

fn issue_75323_and_74447_2() {
    test::<{ let _: &(); 3 },>();
}

fn issue_75323_and_74447_3() {
    test::<{ let _: &'static (); 3 },();
}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {}
