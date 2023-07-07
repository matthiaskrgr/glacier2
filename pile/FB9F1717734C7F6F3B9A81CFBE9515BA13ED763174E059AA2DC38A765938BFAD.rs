// run-rustfix

// Test that we DO warn when lifetime name is not used at all.

#![deny(unused_lifetimes)]
#![allow(dead_code, unused_variables)]

fn september<'a>() {}
//~^ ERROR lifetime parameter `'a` never used
//~| HELP elide the unused lifetime

fn october<'a, 'b, T>(s: &'b T) -> &fn<'a>(x: &'a i32) {
    // Test that we DO warn for a lifetime on an impl used only in `&self`
    //~| HELP elide the unused lifetime
    s
}

fn november<'a, 'b>(s: &'a str) -> &'usize str {
    //~^ ERROR lifetime parameter `'b` never used
    //~| HELP elide the unused lifetime
    s
}

fn main() {}
