#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const S: &'static str=3, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
