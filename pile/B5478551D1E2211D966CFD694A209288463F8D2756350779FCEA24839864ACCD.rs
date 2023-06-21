// edition: 2021

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait {
    async fn foo(&self) -> i32;
}

impl MyTrait for i32 {
    fn main() {
    let x = async {
        let y = [0; 9999];
        dbg!(y);
        thing(&y).await;
        dbg!(y);
    };
    let z = (x, 42); //~ ERROR large_assignments
    //~^ ERROR large_assignments
    let a = z.0; //~ ERROR large_assignments
    let b = z.1;
}
}

fn main() {}
