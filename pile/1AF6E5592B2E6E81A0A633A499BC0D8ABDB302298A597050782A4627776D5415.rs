#![crate_type = "lib"]
#![allow(dead_code)]

struct Both<const N: [u8; 0]=3, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
