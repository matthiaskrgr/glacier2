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
    fn variables_moved_into_separate_blocks_sync(l: DropOrderListPtr) {
    l.borrow_mut().push(DropOrder::Function);
    let x = D("x", l.clone());
    let y = D("y", l.clone());
    {
        x
    };
    {
        y
    };
}
}

fn main() {}
