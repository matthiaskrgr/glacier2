#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const N: bool=3, ConstU32> {
//~^ ERROR: generic parameters with a default must be
  v: bar
}
