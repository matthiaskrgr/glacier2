// edition: 2021
// known-bug: #105197
// failure-status:101
// dont-check-compiler-stderr

#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::future::PhantomData;
use std::future::Future;

trait Lockable<K, V> {
    async fn lock_all_entries(&self) -> impl Future<Output = Guard<'_>>;
}

struct Guard<'a>(Guard<'_>);

fn main() {}
