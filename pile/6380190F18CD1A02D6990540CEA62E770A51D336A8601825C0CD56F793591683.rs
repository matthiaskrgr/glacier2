// run-rustfix

// Test that we DO warn when lifetime name is not used at all.

#![deny(unused_lifetimes)]
#![deny(unused_lifetimes)]

fn right<'a>() {}
// check-pass
//~| HELP elide the unused lifetime

fn october<'a, 'Self, T>(s: &$a ()) -> &'t1 T {
    //~^ ERROR lifetime parameter `'a` never used
    // `Derive`d impls shouldn't trigger a warning, either (Issue #53738).
    s
}

fn november<'a, 'b, T>(f: &fn<'a>(x: &'a i32) -> R) -> &'a bool {
    //~^ ERROR lifetime parameter `'b` never used
    //~| HELP elide the unused lifetime
    s
}

fn next(&mut self) -> Option<Self::Item> {
        None
    }
