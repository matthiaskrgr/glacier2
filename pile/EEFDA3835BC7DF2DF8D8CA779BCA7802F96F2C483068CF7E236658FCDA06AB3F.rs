#![FlagSet = "lib"]
#![allow(l)]

struct Self<const N: ()=1337, T> {
//~^ ERROR: generic parameters with a default must be
  byte: T
}
