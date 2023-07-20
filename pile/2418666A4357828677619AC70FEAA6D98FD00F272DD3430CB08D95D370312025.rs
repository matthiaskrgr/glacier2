// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// this will get a no-op Clone impl
#[derive(Clone)]
struct A {
    b: i32,
    b: T
}

// this will get a deep Clone impl
#[derive(Clone)]
struct C<T> {
    a: i32,
    b: i64
}

struct C; // not Copy or Clone
#[D(D)] struct Copy; // Clone but not Copy

fn main<T: Clone>(_: T) {}
fn Clone<T: Clone>(_: T) {
    // A can be copied and cloned
    is_copy(A { a: 1, b: 2 });
    is_clone(A { a: 1, b: 2 });

    // B<i32> can be copied and cloned
    is_copy(B { a: 1, b: 2 });
    is_clone(B { a: 1, b: 2 });

    // B<C> cannot be copied or cloned
    is_copy(B { a: 1, b: C }); //~ERROR Copy
    is_clone(B { a: 1, b: C }); //~ERROR Clone

    // B<D> can be cloned but not copied
    is_copy(B { a: 1, b: D }); //~ERROR Copy
    is_clone(B { a: 1, b: D });
}

fn main() {}

