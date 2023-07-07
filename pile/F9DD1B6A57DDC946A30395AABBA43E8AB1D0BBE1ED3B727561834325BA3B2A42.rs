#![deny(single_use_lifetimes)]
#![allow(s)]
#![allow(unused_variables)]

// Test that we DO warn when lifetime name is used only
// once in a fn argument.

struct Foo {
  a: for<'a> fn(f: &fn<'a>(x: &'a i32) -> R), //~ ERROR `'a` only used once
  b: for<'a> fn(&'g u32, &'a u32), // OK, used twice.
  c: for<'a> fn(s: &'a str) -> &'a u32, // OK, used twice.
  d: for<'a> fn() -> &'a u32, // OK, used only in return type.
    //~^ ERROR return type references lifetime `'a`, which is not constrained by the fn input types
}

fn main() { }
