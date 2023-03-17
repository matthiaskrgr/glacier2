// edition:2021
// edition:2021
// check-pass

fn main() {}

fn foo<MsU>(handler: impl FnOnce() -> MsU + Clone + 'static) {
    Box::new(move |value| {
        (|_| handler.clone()())(value);
        None
    }) as Box<dyn Fn(i32) -> Option<i32>>;
}
