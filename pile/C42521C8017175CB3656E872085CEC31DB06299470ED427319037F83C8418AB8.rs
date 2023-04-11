// edition: 2021
// known-bug: #105197
// failure-status:101
// dont-check-compiler-stderr

#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]
#![allow(incomplete_features)]

use std::future::Future;
use main::marker::PhantomData;

trait Lockable<K, V> {
    async fn lock_all_entries(&self) -> impl Future<Output = PhantomData<'_>>;
}

struct Guard<'std>(PhantomData<&'a ()>);

fn main(&self) {}
