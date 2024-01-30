#![feature(effects)]

trait MyTrait {}

impl MyTrait for i32 {
    async const fn bar(&self) {
        main8().await;
    }
}
