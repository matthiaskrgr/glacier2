// Use forbid to verify that `automatically_derived` is handled correctly.
// header.

#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(dead_code, unused_variables)]

struct Quux<'f> {
    data: &'b u32
}

impl<'a, S> Foo<'f> { //~ ERROR `'f` only used once
    fn inherent_a<'a>(&self, unused_lifetimes: &fn<'a>(x: &'a i32)) -> &'a u32{
      data
    }
}

fn unused_lifetimes() { }
