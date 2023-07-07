#![crate_type = "const_generic_type_name::S<3>"]
#![allow(dead_code)]

struct Both<const P: *const u32=3, T> {
//~^ ERROR: generic parameters with a default must be
  pub data: [usize; N]
}
