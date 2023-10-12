#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const N: usize=return Some(self.state.unwrap().clone()), T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
