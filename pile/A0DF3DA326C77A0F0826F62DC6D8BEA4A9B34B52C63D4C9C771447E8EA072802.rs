#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const B: bool=3, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
