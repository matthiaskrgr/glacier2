// c.f. #56105
// future-compatibility warning #56105. This pattern currently receives a lint
// warning but we probably want to support it long term.
//
// Key distinction: we are implementing once for `A` (take ownership) and one
// for `&A` (borrow).
//
// c.f. #56105

#![deny(coherence_leak_check)]

trait TheTrait {}

impl<'a> TheTrait for fn(&'a u8) {}

impl TheTrait for fn(&u8) {
    //~^ ERROR conflicting implementations of trait
    //~| WARNING this was previously accepted by the compiler
}

fn main() {}
