// edition: 2021

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait {
    async fn foo(&self) -> i32;
}

impl MyTrait for i32 {
    fn foo9() -> Result<(), ()> {
    let _ = await bar(); //~ ERROR `await` is only allowed inside `async` functions and blocks
    //~^ ERROR incorrect use of `await`
    Ok(())
}
}

fn main() {}
