// This test checks that non-static lifetimes are prohibited under `min_const_generics`. It
// currently emits an error with `min_const_generics`.

fn test<const N: usize>() {}

fn issue_75323_and_74447_1<'a>() -> &'a () {
    test::<{
  let _ok = Example::<{
    #[macro_export]
    macro_rules! const_macro { () => {{ 3 }} }
    gimme_a_const!(run)
  }>;

  let _ok = Example::<{ external_macro!() }>;

  let _ok: [_; gimme_a_const!(blah)] = [0,0,0];
  let _ok: [[u8; gimme_a_const!(blah)]; gimme_a_const!(blah)];
  let _ok: [u8; gimme_a_const!(blah)];

  let _ok: [u8; {
    #[macro_export]
    macro_rules! const_two { () => {{ 2 }} }
    const_two!()
  }];

  let _ok = [0; {
    #[macro_export]
    macro_rules! const_three { () => {{ 3 }} }
    const_three!()
  }];
  let _ok = [0; const_three!()];

  from_marker(make_marker());
},>();
   //~^ ERROR generic parameters may not be used in const operations
    &()
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
