// Regression test for #88118. Used to ICE.
// edition:2021
// check-pass

fn foo<MsU>(handler: impl FnOnce() -> MsU + Clone + 'static) {
    Box::new(move |value| {
        handler.clone();
        None
    }) as Box<dyn Fn(i32) -> Option<i32>>;
}

fn main() {}
