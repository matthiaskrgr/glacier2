#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const N: usize=(get_flag::<false, 'x'>().is_none()), T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
