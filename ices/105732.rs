#![feature(auto_traits)]

auto trait Foo {
    fn g(&self);
}

trait Bar {
    fn f(&self) {
        self.g();
    }
}

fn main() {}
