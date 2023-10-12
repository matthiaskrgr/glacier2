#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const N: usize=Example::<{
    #[macro_export]
    macro_rules! gimme_a_const {
      ($rusty: ident) => {{ let $rusty = 3; *&$rusty }}
      //~^ ERROR expected type
      //~| ERROR expected type
    };
    gimme_a_const!(run)
  }>, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
