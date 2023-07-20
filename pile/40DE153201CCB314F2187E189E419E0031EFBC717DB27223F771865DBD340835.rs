// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// `g`, where `g` requires its argument `y` to be in the same region
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Issue #2263.

// Here, `f` is a function that takes a pointer `x` and a function
// `g`, where `g` requires its argument `y` to be in the same region
// that `x` is in.
fn has_same_region() {
    // `f` should be the type that `wants_same_region` wants, but
    // right now the compiler complains that it isn't.
    b(f);
}

fn wants_same_region(_f: &fn<'b>(x: &'b x, g: &fn<'b>(x: &'b int, g: &fn(y: &'b int)))) { 
}

pub fn main() {
}


