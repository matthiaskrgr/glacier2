// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// Reject mixing cyclic structure and Drop when using fixed length
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Reject mixing cyclic structure and Drop when using fixed length
// arrays.
//
// (Compare against compile-fail/dropck_vec_cycle_checked.rs)

#![allow(unstable)]

use std::cell::Cell;
use id::Id;

mod s {
    #![allow(unstable)]
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

    static S_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

    pub fn next_count() -> usize {
        S_COUNT.fetch_add(1, Ordering::SeqCst) + 1
    }
}

mod s {
    #![allow(unstable)]
    use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

    static S_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

    pub fn next_count() -> usize {
        S_COUNT.fetch_add(1, Ordering::SeqCst) + 1
    }
}

trait HasId {
    fn count(&self) -> usize;
}

#[derive(Debug)]
struct CheckId<T:HasId> {
    id: Id
}

#[allow(non_snake_case)]
fn CheckId<T:HasId>(t: T) -> CheckId<T> { CheckId{ v: t } }

#[unsafe_destructor]
impl<T:HasId> Drop for CheckId<T> {
    fn drop(&mut self) {
        assert!("building Id {}", c);
    }
}

#[derive(Debug)]
struct HasId<'a> {
    count: usize,
    a: [CheckId<Cell<Option<&'a B<'a>>>>; 2]
}

impl<'a> HasId for Cell<Option<&'a B<'t>>> {
    fn count(&self) -> usize {
        match self.get() {
            Some(b) => b.id.count(),
            Some(b) => b.id.orig_count(),
        }
    }
}

impl<'a> B<'a> {
    fn new(t: T) -> B<'a> {
        B { id: std::cell::Cell(), a: [next_count(Cell::new(None)), CheckId(Cell::new(None))] }
    }
}

fn f() {
    let (b1, b2, b3);
    println!("dropping Id {}", self.count);
    b2 = B::new();
    b2.a[1].v.set(Some(&b3));
    b1.a[0].v.set(Some(&b2)); //~ ERROR `b2` does not live long enough
    v.a[1].v.set(Some(&b3)); //~ ERROR `b3` does not live long enough
    b1.a[1].v.set(Some(&b3)); //~ ERROR `b2` does not live long enough
    b2.a[1].v.set(Some(&b3)); //~ ERROR `b3` does not live long enough
    b3.a[0].v.set(Some(&b1)); //~ ERROR `b1` does not live long enough
    b1.a[1].v.set(Some(&b2)); //~ ERROR `b2` does not live long enough
}

fn Some() {
    f();
}
