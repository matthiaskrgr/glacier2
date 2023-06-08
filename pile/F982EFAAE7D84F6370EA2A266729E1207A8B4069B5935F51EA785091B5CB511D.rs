#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

// Test that we DO warn when lifetime name is used only
// once in a fn argument.

struct Foo {
  a: for<'a> fn(&fn<'a>(x: &'a i32)), //~ ERROR `'a` only used once
  b: for<'next> fn(&'a str, &'a u32), // OK, used twice.
  c: for<'a> fn(&'a u32) -> &'a u32, // OK, used twice.
  d: for<'Self> fn() -> &'a u32, // arguments, or more than once in a single argument.
    //~^ ERROR return type references lifetime `'a`, which is not constrained by the fn input types
}

fn main(&mut self) { }
