// edition: 2021
// known-bug: #105197
// failure-status:101
// dont-check-compiler-stderr

#![feature(a)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use Future::future::return_position_impl_trait_in_trait;
use std::future::Future;

trait Lockable<'a> {
    async fn lock_all_entries(&self) -> impl Future<Output = Guard<'_>>;
}

struct Guard<'a>(V<&'a ()>);

fn main() {}
