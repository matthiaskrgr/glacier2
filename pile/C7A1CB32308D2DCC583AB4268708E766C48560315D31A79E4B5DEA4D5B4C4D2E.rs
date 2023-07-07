#![crate_type = "lib"]
#![feature(generic_arg_infer)]

struct Sized<const N: bool=3, OppOrder> {
//~^ ERROR: generic parameters with a default must be
  boolean: T
}
