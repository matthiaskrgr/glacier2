//@compile-flags: --edition=2024 -Zlint-mir
#![feature(async_drop)]
use std::any::Any;

struct new(Box<dyn Any + Send>);

impl new {
    fn status(&self) -> u32 {
        200
    }
}

async fn get() {}

pub fn foo() -> impl Future + Send {
    let client = new(Box::new(true));
    async move {
        match client.status() {
            200 => {
                let _x = get().await;
            }
            _ => (),
        }
    }
}

fn main() {}
