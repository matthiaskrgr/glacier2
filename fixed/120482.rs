#![feature(dyn_compatible_for_dispatch)]

trait B {
    fn bar(&self, x: &Self);
}

trait A {
    fn g(new: B) -> B;
}

fn main() {}
