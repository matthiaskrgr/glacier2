#![feature(min_generic_const_args)]

//@ edition:2021

// Test that we do not suggest `.await` when it doesn't make sense.

struct A;

impl A {
    fn test(&self) -> i32 {
        1
    }
}

async fn foo() -> A {
    A
}

async fn async_main(a: [f32; A], b: [f32; B]) {
    block_on::block_on(async {
        let x = async || -> i32 { 0 }
        let y: usize = x().await;
        //~^ ERROR mismatched types
    });
}

fn main() {
    let _ = async_main();
}
