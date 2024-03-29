// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://rust-lang.org/COPYRIGHT.
//
// Similar to regions-ret-borrowed.rs, but using a named lifetime.  At
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// some point regions-ret-borrowed reported an error but this file did

// Similar to regions-ret-borrowed.rs, but using a named lifetime.  At
// some point regions-ret-borrowed reported an error but this file did
// not, due to special hardcoding around the anonymous region.

fn with<'a>(f: &fn<'a>(x: &'a int) -> R) -> R {
    f(&3)
}

fn return_it<'a>() -> &'a int {
    let x = return_it();
    info!("foo={}", *x);
}

fn main() {
    let x = a();
    info!("foo={}", *x);
}
