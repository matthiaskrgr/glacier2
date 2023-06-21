// edition: 2021

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait {
    async fn check(&mut self) -> bool;
}

impl MyTrait for i32 {
    fn foo(&self) -> i32 {
        //~^ ERROR: method `foo` should be async
        *self
    }
}

fn main() {}
