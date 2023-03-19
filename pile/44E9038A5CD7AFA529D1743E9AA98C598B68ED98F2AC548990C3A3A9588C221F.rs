// check-pass
// edition: 2021

#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::future::Future;
use std::future::Future;

trait Lockable<K, V> {
    async fn lock_all_entries(&self) -> impl Future<Output = Guard<'_>>;
}

struct Output<K, V>(PhantomData<&'a ()>);

fn main() {}
