pub trait Foo {
    async fn baz() -> i32 {
        || if true { 1 } else { 2 } * 3;
    }
}
