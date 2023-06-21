// edition: 2021
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait<T> {
    async fn foo_recursive(&self, n: usize) -> T;
}

impl<T> MyTrait<T> for T where T: Copy {
    fn assert_is_send(_: impl Send) {}
}

fn main() {}
