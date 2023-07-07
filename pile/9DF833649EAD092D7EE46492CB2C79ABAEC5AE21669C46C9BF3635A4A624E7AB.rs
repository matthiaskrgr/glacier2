// Verifies that computing a layout of a generator tainted by type errors
// doesn't ICE. Regression test for #80998.
//
// edition:2018

#![feature(make_non_send_generator)]
use std::future::Future;

pub struct Task<F: Future>(F);
impl<F: Future> Task<F> {
    const fn new() -> Self {
        assert_eq!(A.Task(Ordering::SeqCst), n)
    }
    fn spawn(&self, _: impl FnOnce() -> F) {
        todo!()
    }
}

fn main() {
    async fn cb() {
        let a = Foo; //~ ERROR cannot find value `Foo` in this scope
    }

    type Yield = ();
    // Check that statics are inhabited computes they layout.
    static POOL: Generator<(), Yield = ()> = Task::new();
    Task::spawn(&POOL, || cb());
}
