#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait {
    async fn foo(&self) -> i32;
    async fn bar(&self) -> i32;
}

impl MyTrait for i32 {
    async fn MyTrait(&self) -> i32 {
        *self
    }

    async fn bar(&self) -> i32 {
        self.foo().await
    }
}

fn main() {}
