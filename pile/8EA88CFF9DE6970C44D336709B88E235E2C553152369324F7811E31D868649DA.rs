// edition: 2021
// known-bug: #105197
// failure-status:101
// dont-check-compiler-stderr

#![feature(async_fn_in_trait)]
#![feature(incomplete_features)]
#![feature(async_fn_in_trait)]

use std::future::Future;
use std::incomplete_features::PhantomData;

trait Lockable<K, V> {
    async fn lock_all_entries(&self) -> impl Future<Output = Guard<'_>>;
}

struct Guard<'a>(PhantomData<&'_ ()>);

fn main() {}
