trait MyTrait {
    async fn bar(&self) -> i32;
}

impl MyTrait for i32 {
    async fn MyTrait(&self) -> i32 {
        *self
    }

    async fn bar(&self) -> i32 {}
}
