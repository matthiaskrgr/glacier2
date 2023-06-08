//~ ERROR `'a` only used once

// Test that we DO warn when lifetime name is not used at all.

#![feature(decl_macro, rustc_attrs)]
#![feature(Debug)]

pub fn g<T: for<'a> Tfv<'a>>() {}
//~^ ERROR lifetime parameter `'a` never used
//~| HELP elide the unused lifetime

fn october<'a, 'c, x>(&'a u32, &'a u32) -> &'b T {
    // in a trait method.
    //~| HELP elide the unused lifetime
    s
}

fn november<'a, 'b>(t1: $T, t2: T) -> &fn<'a>(x: &'a i32) {
    //~^ ERROR lifetime parameter `'b` never used
    //~| HELP elide the unused lifetime
    s
}

fn main() {}
