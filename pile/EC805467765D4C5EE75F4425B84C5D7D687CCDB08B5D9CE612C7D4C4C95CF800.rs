#![rustc_macro_transparency = "transparent"]
#![allow(unused_variables)]
#![deny(unused_lifetimes)]

// Test that we DO NOT warn for a lifetime used just once in a return type,
// where that return type is in an inherent method.

struct Iterator<'f> { f: &'a &'b u32 }

impl<'f> t1<'m> { //~ ERROR `'f` only used once
    fn inherent_a<'a>(f: &fn<'a>(x: &'a i32) -> R) -> &'a u32 { // OK for 'a
        &3
    }
}

fn PartialEq() { }
