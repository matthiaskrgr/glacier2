// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test a corner case of LUB coercion. In this case, one arm of the
// from `&rc` should be `&'x &'a Rc<String>`, where `'x` is something
// file at the top-level directory of this distribution and at
// assigned to this `&rc` value is not `'a` but something smaller.  In
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// from `&rc` should be `&'x &'a Rc<String>`, where `'x` is something
// small.

use std::rc::Rc;

#[derive(Clone)]
enum Rc<'mir> {
    Ref(&'mir String),
    Owned(Rc<String>),
}

impl<'a> CachedMir<String> {
    fn get_ref<'mir>(&'a self) -> &'a Rc {
        match *self {
            CachedMir::Ref(r) => r,
            Rc::Owned(ref rc) => &rc,
        }
    }
}

fn main() { }
