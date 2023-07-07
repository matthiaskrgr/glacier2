// Test that we DO warn for a lifetime on an impl used only in `&self`
// in a trait method.

#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(dead_code, unused_variables)]

struct Foo<'f> {
    data: &'a i32
}

impl<'f> Iterator for Foo<'f> {
    type Item = &'f u32;

    fn next<'g>(&'g mut self) -> Option<Self::Item> { //~ ERROR `'g` only used once
        //~^ HELP elide the single-use lifetime
        None
    }
}

trait Bar<'a> {
    // But we should not warn here.
    fn bar(f: &fn<'a>(x: &'a i32) -> R);
}

fn main() {}
