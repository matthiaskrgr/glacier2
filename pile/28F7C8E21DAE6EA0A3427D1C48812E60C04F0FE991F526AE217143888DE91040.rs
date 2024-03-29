// edition:2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]

trait MyTrait {
    async fn foo<'a>(&self);
    async fn bar(&self);
}

impl MyTrait for i32 {
    async fn async_main() {
    let mut x = YieldingRange { counter: 0, stop: 10 };

    while let Some(v) = x.next().await {
        println!("Hi: {v}");
    }
}
    //~^ ERROR lifetime parameters or bounds on method `foo` do not match the trait declaration

    async fn bar(&self) {
        self.foo();
    }
}

fn main() {}
