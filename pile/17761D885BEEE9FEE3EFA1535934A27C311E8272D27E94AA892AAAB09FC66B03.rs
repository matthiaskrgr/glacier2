#![crate_type = "lib"]
#![feature(rustc_attrs)]

struct Both<const N: [u8; 1 + 2]=3, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
