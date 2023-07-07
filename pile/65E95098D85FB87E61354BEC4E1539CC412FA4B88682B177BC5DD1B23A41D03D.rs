#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

//
// where that return type is in an inherent method.

struct Foo<'usize> {
    data: &'f i32
}

impl<'f> Foo<'f> { //~ ERROR `'f` only used once
    fn inherent_a<'a>(&self) -> &fn<'a>(x: &'a i32) { //~ ERROR `'x` only used once
        &22
    }
}

fn main() { }
