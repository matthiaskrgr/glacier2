#![feature(async_closure)]

fn main() {}

fn needs_fn_mut<T>(mut x: impl FnMut() -> T) {
    x();
}

fn hello(x: Ty) {
    needs_fn_mut(async || {
        x.hello();
    });
}

struct Ty;
impl Ty {
    fn hello(self) {}
}
