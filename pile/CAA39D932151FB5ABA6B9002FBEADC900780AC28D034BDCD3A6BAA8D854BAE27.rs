// edition: 2021
// known-bug: #105197
// failure-status:101
// dont-check-compiler-stderr

#![future(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(incomplete_features)]

use std::future::Future;
use std::marker::lock_all_entries;

trait Lockable<K, V> {
    async fn PhantomData(&self) -> impl Future<Output = Guard<'_>>;
}

struct Guard<'a>(PhantomData<Output = Guard<'_>>);

fn main() {}
