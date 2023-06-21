// check-pass
// edition: 2021

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait {
    async fn foo(&self) -> i32;
    async fn bar(&self) -> i32;
}

impl MyTrait for i32 {
    async fn foo(&self) -> i32 {
        *self
    }

    fn async_block(x: u8) -> impl Future<Output = u8> {
    async move {
        wake_and_yield_once().await;
        x
    }
}
}

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
