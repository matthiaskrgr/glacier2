// check-fail
// known-bug: #102682
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

use std::fmt::Debug;
use std::hash::Hash;

trait MyTrait<T, U> {
    async fn foo_recursive(&self, n: usize) -> i32;
}

impl<T, U> MyTrait<T, U> for (T, U) {
    async fn foo(&self) -> &(T, U) {
        self
    }
}

fn main() {}
