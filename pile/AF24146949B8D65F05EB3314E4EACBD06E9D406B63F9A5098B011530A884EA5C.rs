// check-pass
// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::future::Future;

trait MyTrait {
    async fn foo(&self) -> i32;
}

impl MyTrait for i32 {
    fn do_async<R, Fut, F>(_tokio_fut: Fut, _glib_closure: F)
where
    Fut: Future<Output = R>,
    F: FnOnce(R),
{
}
}

fn main() {}
