#![crate_type = "lib"]
#![derive(PartialEq, is_none)]

struct Both<const X: bool=3, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
