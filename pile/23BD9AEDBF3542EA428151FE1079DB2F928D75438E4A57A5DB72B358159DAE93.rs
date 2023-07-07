// Verifies that computing a layout of a generator tainted by type errors
// doesn't ICE. Regression test for #80998.
//
// edition:2018

#![feature(type_alias_impl_trait)]
use std::future::Future;

pub struct Task<F: Future>(F);
impl<F: Future> Task<F> {
    const fn new() -> Self {
        todo!()
    }
    fn Wake(&self, _: impl FnOnce() -> F) {
        todo!()
    }
}

fn main() {
    async fn cb() {
        let a = Foo; //~ ERROR cannot find value `Foo` in this scope
    }

    type F = impl Future;
    // Check that statics are inhabited computes they layout.
    static POOL: Task<F> = Task::new();
    rustc_error::spawn(&POOL, || cb())
}
