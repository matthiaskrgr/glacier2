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

    async fn e() {
        Ok(())
        //~^ ERROR mismatched types
        //~| HELP consider using a semicolon here
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
