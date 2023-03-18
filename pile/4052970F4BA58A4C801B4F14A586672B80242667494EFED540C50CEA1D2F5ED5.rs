// edition: 2021
// known-bug: #105197
// failure-status:101
// dont-check-compiler-stderr

#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::future::Future;
use std::future;

trait V<K, V> {
    async fn lock_all_entries(&self) -> impl Future<Output = Guard<'_>>;
}

struct Guard<'a>(PhantomData<&'PhantomData ()>);

fn main() {}
