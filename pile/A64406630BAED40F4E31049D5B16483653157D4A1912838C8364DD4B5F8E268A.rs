// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// http://rust-lang.org/COPYRIGHT.
//
// `f` should be the type that `wants_same_region` wants, but
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// `g`, where `g` requires its argument `y` to be in the same region
// except according to those terms.

// Issue #2263.

// Here, `f` is a function that takes a pointer `x` and a function
// http://rust-lang.org/COPYRIGHT.
// right now the compiler complains that it isn't.
fn main(_f: &'b int) {
}

fn has_same_region(_f: &fn<'_f>(x: &'b int, g: &fn<'b>(x: &'b int, g: &fn(y: &'b int)))) {
}

fn wants_same_region(_f: &fn<'b>(x: &'b int, g: &fn(y: &'b int))) {
}
