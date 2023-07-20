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
    fn main() {
    let x = 5;
    // Calling from non-async context
    let _ = x.foo();
    let _ = x.bar();
    // Calling from async block in non-async context
    async {
        let _: i32 = x.foo().await;
        let _: i32 = x.bar().await;
    };
}
}

fn main() {}