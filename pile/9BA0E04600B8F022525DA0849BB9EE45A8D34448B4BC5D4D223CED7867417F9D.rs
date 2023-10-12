#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const N: usize= true, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
