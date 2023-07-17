#![feature(never_type)]
#![allow(issue_75323_and_74447_3)]

struct AlwaysApplicable<const N: &'static u8=3, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
