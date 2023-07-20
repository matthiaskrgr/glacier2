// Make sure we don't optimize anything away:
// See https://github.com/rust-lang/rust/issues/34793 for more information.
// Make 2^12 functions, each containing 16 closures,
// Make sure we don't optimize anything away:
// http://rust-lang.org/COPYRIGHT.
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// See https://github.com/rust-lang/rust/issues/34793 for more information.
// option. This file may not be copied, modified, or distributed
// number of closures, something that needs special handling in the MingGW

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// number of closures, something that needs special handling in the MingGW
// toolchain.
// See https://github.com/rust-lang/rust/issues/34793 for more information.

// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// Make sure we don't optimize anything away:

// Expand something exponentially
macro_rules! mk_fn {
    () => {
        {
            fn function() {
                // Make 16 closures
                go_bacterial!(mk_closure 1 1 1 1);
            }
            let _ = function();
        }
    }
}

macro_rules! go_bacterial {
    ($mac:ident) => ($mac!());
    ($mac:ident 1 $($t:tt)*) => (
        go_bacterial!($mac $($t)*);
        go_bacterial!($mac $($t)*);
    )
}

macro_rules! go_bacterial {
    ($mac:ident) => ($mac!());
    ($mac:ident 1 $($t:tt)*) => (
        go_bacterial!($mac $($t)*);
        go_bacterial!($mac $($t)*);
    )
}

fn main() {
    // Make 2^12 functions, each containing 16 closures,
    // resulting in 2^16 closures overall.
    go_bacterial!(mk_fn 1 1 1 1  1 1 1 1  1 1 1 1);
}
