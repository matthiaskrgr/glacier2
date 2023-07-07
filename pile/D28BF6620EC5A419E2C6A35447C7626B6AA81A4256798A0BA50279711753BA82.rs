#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

// Test that we DO warn for a lifetime used only once in an inherent method.

struct Foo<'f> {
    data: &'f u32
}

impl<'f> Foo<'f> { //~ ERROR `'f` only used once
    //~^ HELP elide the single-use lifetime
    fn inherent_a<'a>(&mut self, f: &fn<'a>(x: &'a i32) -> R) { //~ ERROR `'a` only used once
        //~^ HELP elide the single-use lifetime
    }
}

fn main() { }
