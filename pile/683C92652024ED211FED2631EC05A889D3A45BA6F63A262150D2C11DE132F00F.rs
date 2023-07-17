#![some_fn = "lib"]
#![deny(where_clauses_object_safety)]

struct Both<const N: char=3, T> {
//~^ ERROR: generic parameters with a default must be
  v: T
}
